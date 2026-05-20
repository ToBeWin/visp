import Foundation
import Speech

enum RecognizerError: Error, CustomStringConvertible {
    case missingPath
    case fileMissing(String)
    case authorizationDenied
    case onDeviceNotSupported(String)
    case noRecognizer(String)
    case noResult
    case recognitionFailed(String)

    var description: String {
        switch self {
        case .missingPath:
            return "缺少音频文件路径参数"
        case .fileMissing(let path):
            return "音频文件不存在: \(path)"
        case .authorizationDenied:
            return "语音识别权限未授权，请在系统设置中允许 Visp 使用 Speech Recognition"
        case .onDeviceNotSupported(let locale):
            return "当前系统未为 \(locale) 提供离线语音识别支持"
        case .noRecognizer(let locale):
            return "无法创建 \(locale) 识别器"
        case .noResult:
            return "没有识别到可用文本"
        case .recognitionFailed(let message):
            return message
        }
    }
}

func speechAuthorized() -> Bool {
    SFSpeechRecognizer.authorizationStatus() == .authorized
}

func authorizeSpeech() throws {
    let semaphore = DispatchSemaphore(value: 0)
    var status: SFSpeechRecognizerAuthorizationStatus = .notDetermined

    SFSpeechRecognizer.requestAuthorization { authStatus in
        status = authStatus
        semaphore.signal()
    }

    semaphore.wait()

    guard status == .authorized else {
        throw RecognizerError.authorizationDenied
    }
}

func emitPermissionStatus() throws {
    let payload = [
        "speech": speechAuthorized(),
    ]
    let data = try JSONSerialization.data(withJSONObject: payload, options: [])
    FileHandle.standardOutput.write(data)
    FileHandle.standardOutput.write(Data("\n".utf8))
}

func recognize(url: URL, localeIdentifier: String) throws -> String {
    guard let recognizer = SFSpeechRecognizer(locale: Locale(identifier: localeIdentifier)) else {
        throw RecognizerError.noRecognizer(localeIdentifier)
    }

    guard recognizer.supportsOnDeviceRecognition else {
        throw RecognizerError.onDeviceNotSupported(localeIdentifier)
    }

    let request = SFSpeechURLRecognitionRequest(url: url)
    request.requiresOnDeviceRecognition = true
    request.shouldReportPartialResults = false
    request.addsPunctuation = true

    let semaphore = DispatchSemaphore(value: 0)
    var finalText: String?
    var finalError: Error?

    let task = recognizer.recognitionTask(with: request) { result, error in
        if let result {
            finalText = result.bestTranscription.formattedString
            if result.isFinal {
                semaphore.signal()
            }
        }

        if let error {
            finalError = error
            semaphore.signal()
        }
    }

    _ = semaphore.wait(timeout: .now() + 30)
    task.cancel()

    if let finalError {
        throw RecognizerError.recognitionFailed(finalError.localizedDescription)
    }

    guard let finalText, !finalText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty else {
        throw RecognizerError.noResult
    }

    return finalText.trimmingCharacters(in: .whitespacesAndNewlines)
}

do {
    let arguments = Array(CommandLine.arguments.dropFirst())

    if arguments.first == "--check-permissions" {
        try emitPermissionStatus()
        exit(0)
    }

    if arguments.first == "--request-permissions" {
        try authorizeSpeech()
        try emitPermissionStatus()
        exit(0)
    }

    guard let audioPath = arguments.first else {
        throw RecognizerError.missingPath
    }

    guard FileManager.default.fileExists(atPath: audioPath) else {
        throw RecognizerError.fileMissing(audioPath)
    }

    try authorizeSpeech()

    let url = URL(fileURLWithPath: audioPath)
    let locales = ["zh-CN", "en-US"]
    var candidates: [String] = []
    var lastError: Error?

    for locale in locales {
        do {
            let text = try recognize(url: url, localeIdentifier: locale)
            candidates.append(text)
        } catch {
            lastError = error
        }
    }

    if let best = candidates.max(by: { $0.count < $1.count }) {
        FileHandle.standardOutput.write(Data(best.utf8))
        FileHandle.standardOutput.write(Data("\n".utf8))
    } else if let lastError {
        throw lastError
    } else {
        throw RecognizerError.noResult
    }
} catch {
    FileHandle.standardError.write(Data((error.localizedDescription + "\n").utf8))
    exit(1)
}
