#import <AVFoundation/AVFoundation.h>
#import <Foundation/Foundation.h>
#import <Speech/Speech.h>
#include <dispatch/dispatch.h>
#include <string.h>

static void visp_write_string(NSString *value, char *buffer, int buffer_len) {
  if (buffer == NULL || buffer_len <= 0) {
    return;
  }

  const char *utf8 = value != nil ? value.UTF8String : "";
  strncpy(buffer, utf8, (size_t)buffer_len - 1);
  buffer[buffer_len - 1] = '\0';
}

int visp_check_microphone_permission(void) {
  AVAuthorizationStatus status = [AVCaptureDevice authorizationStatusForMediaType:AVMediaTypeAudio];
  return status == AVAuthorizationStatusAuthorized ? 1 : 0;
}

int visp_get_microphone_permission_status(void) {
  return (int)[AVCaptureDevice authorizationStatusForMediaType:AVMediaTypeAudio];
}

int visp_request_microphone_permission(char *error, int error_len) {
  @autoreleasepool {
    AVAuthorizationStatus status = [AVCaptureDevice authorizationStatusForMediaType:AVMediaTypeAudio];
    if (status == AVAuthorizationStatusAuthorized) {
      return 1;
    }

    if (status == AVAuthorizationStatusDenied || status == AVAuthorizationStatusRestricted) {
      visp_write_string(@"麦克风权限未授权，请在系统设置中允许 Visp 使用麦克风", error, error_len);
      return 0;
    }

    dispatch_semaphore_t semaphore = dispatch_semaphore_create(0);
    __block BOOL granted = NO;

    [AVCaptureDevice requestAccessForMediaType:AVMediaTypeAudio
                             completionHandler:^(BOOL allowed) {
                               granted = allowed;
                               dispatch_semaphore_signal(semaphore);
                             }];

    dispatch_semaphore_wait(semaphore, DISPATCH_TIME_FOREVER);

    if (!granted) {
      visp_write_string(@"麦克风权限未授权，请在系统设置中允许 Visp 使用麦克风", error, error_len);
      return 0;
    }

    return 1;
  }
}

int visp_check_speech_permission(void) {
  SFSpeechRecognizerAuthorizationStatus status = [SFSpeechRecognizer authorizationStatus];
  return status == SFSpeechRecognizerAuthorizationStatusAuthorized ? 1 : 0;
}

int visp_get_speech_permission_status(void) {
  return (int)[SFSpeechRecognizer authorizationStatus];
}

int visp_request_speech_permission(char *error, int error_len) {
  @autoreleasepool {
    SFSpeechRecognizerAuthorizationStatus status = [SFSpeechRecognizer authorizationStatus];
    if (status == SFSpeechRecognizerAuthorizationStatusAuthorized) {
      return 1;
    }

    if (status == SFSpeechRecognizerAuthorizationStatusDenied ||
        status == SFSpeechRecognizerAuthorizationStatusRestricted) {
      visp_write_string(@"语音识别权限未授权。请在系统设置的 Speech Recognition 中允许 Visp；如果列表里暂时没有 Visp，请回到应用前台再次点击授予权限。", error, error_len);
      return 0;
    }

    dispatch_semaphore_t semaphore = dispatch_semaphore_create(0);
    __block SFSpeechRecognizerAuthorizationStatus result = SFSpeechRecognizerAuthorizationStatusNotDetermined;

    [SFSpeechRecognizer requestAuthorization:^(SFSpeechRecognizerAuthorizationStatus authStatus) {
      result = authStatus;
      dispatch_semaphore_signal(semaphore);
    }];

    dispatch_semaphore_wait(semaphore, DISPATCH_TIME_FOREVER);

    if (result != SFSpeechRecognizerAuthorizationStatusAuthorized) {
      visp_write_string(@"语音识别权限未授权。请在系统设置的 Speech Recognition 中允许 Visp；如果列表里暂时没有 Visp，请回到应用前台再次点击授予权限。", error, error_len);
      return 0;
    }

    return 1;
  }
}

static NSString *visp_recognize_locale(NSURL *url, NSString *locale_identifier, NSString **error_message) {
  NSLocale *locale = [[NSLocale alloc] initWithLocaleIdentifier:locale_identifier];
  SFSpeechRecognizer *recognizer = [[SFSpeechRecognizer alloc] initWithLocale:locale];
  if (recognizer == nil) {
    *error_message = [NSString stringWithFormat:@"无法创建 %@ 识别器", locale_identifier];
    return nil;
  }

  if (![recognizer supportsOnDeviceRecognition]) {
    *error_message = [NSString stringWithFormat:@"当前系统未为 %@ 提供离线语音识别支持", locale_identifier];
    return nil;
  }

  SFSpeechURLRecognitionRequest *request = [[SFSpeechURLRecognitionRequest alloc] initWithURL:url];
  request.requiresOnDeviceRecognition = YES;
  request.shouldReportPartialResults = NO;

  dispatch_semaphore_t semaphore = dispatch_semaphore_create(0);
  __block NSString *final_text = nil;
  __block NSString *local_error = nil;

  SFSpeechRecognitionTask *task =
      [recognizer recognitionTaskWithRequest:request
                               resultHandler:^(SFSpeechRecognitionResult *_Nullable result,
                                               NSError *_Nullable recognition_error) {
                                 if (result != nil) {
                                   final_text = result.bestTranscription.formattedString;
                                   if (result.isFinal) {
                                     dispatch_semaphore_signal(semaphore);
                                   }
                                 }

                                 if (recognition_error != nil) {
                                   local_error = recognition_error.localizedDescription;
                                   dispatch_semaphore_signal(semaphore);
                                 }
                               }];

  long wait_result = dispatch_semaphore_wait(semaphore, dispatch_time(DISPATCH_TIME_NOW, 30 * NSEC_PER_SEC));
  [task cancel];

  if (wait_result != 0) {
    *error_message = @"语音识别超时";
    return nil;
  }

  if (local_error != nil) {
    *error_message = local_error;
    return nil;
  }

  NSString *trimmed = [final_text stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceAndNewlineCharacterSet]];
  if (trimmed == nil || trimmed.length == 0) {
    *error_message = @"没有识别到可用文本";
    return nil;
  }

  return trimmed;
}

int visp_transcribe_file(const char *path,
                         char *output,
                         int output_len,
                         char *error,
                         int error_len) {
  @autoreleasepool {
    if (path == NULL) {
      visp_write_string(@"缺少音频文件路径参数", error, error_len);
      return 0;
    }

    if ([SFSpeechRecognizer authorizationStatus] != SFSpeechRecognizerAuthorizationStatusAuthorized) {
      visp_write_string(@"语音识别权限未授权。请先回到 Visp 前台重新触发授权；如果系统设置里仍然没有 Visp，说明系统还没登记这次权限请求。", error, error_len);
      return 0;
    }

    NSString *audio_path = [NSString stringWithUTF8String:path];
    if (![[NSFileManager defaultManager] fileExistsAtPath:audio_path]) {
      visp_write_string([NSString stringWithFormat:@"音频文件不存在: %@", audio_path], error, error_len);
      return 0;
    }

    NSURL *url = [NSURL fileURLWithPath:audio_path];
    NSArray<NSString *> *locales = @[ @"zh-CN", @"en-US" ];
    NSMutableArray<NSString *> *candidates = [NSMutableArray array];
    NSString *last_error = nil;

    for (NSString *locale_identifier in locales) {
      NSString *locale_error = nil;
      NSString *text = visp_recognize_locale(url, locale_identifier, &locale_error);
      if (text != nil) {
        [candidates addObject:text];
      } else if (locale_error != nil) {
        last_error = locale_error;
      }
    }

    if (candidates.count == 0) {
      visp_write_string(last_error != nil ? last_error : @"没有识别到可用文本", error, error_len);
      return 0;
    }

    NSString *best = candidates[0];
    for (NSString *candidate in candidates) {
      if (candidate.length > best.length) {
        best = candidate;
      }
    }

    visp_write_string(best, output, output_len);
    return 1;
  }
}
