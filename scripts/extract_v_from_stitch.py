#!/usr/bin/env python3
"""
从 Stitch 2.0 设计的截图中精确提取 V 字母
去掉底部的 "Visp" 文字，只保留 V
"""

from PIL import Image, ImageDraw, ImageFilter
import os

def extract_icon_and_remove_text(input_path):
    """
    从截图中提取图标，并去掉底部文字
    """
    print(f"📖 读取 Stitch 2.0 设计: {input_path}")
    img = Image.open(input_path)
    
    if img.mode != 'RGBA':
        img = img.convert('RGBA')
    
    width, height = img.size
    print(f"   原始尺寸: {width}x{height}")
    
    import numpy as np
    img_array = np.array(img)
    
    # 计算亮度
    brightness = img_array[:, :, :3].mean(axis=2)
    
    # 找到图标区域（亮度较高的矩形区域）
    threshold = brightness.mean() + brightness.std() * 0.5
    binary = brightness > threshold
    
    rows_with_icon = np.where(binary.any(axis=1))[0]
    cols_with_icon = np.where(binary.any(axis=0))[0]
    
    if len(rows_with_icon) > 0 and len(cols_with_icon) > 0:
        # 获取图标边界
        top = rows_with_icon[0]
        bottom = rows_with_icon[-1]
        left = cols_with_icon[0]
        right = cols_with_icon[-1]
        
        # 添加小边距
        margin = 5
        top = max(0, top - margin)
        bottom = min(height - 1, bottom + margin)
        left = max(0, left - margin)
        right = min(width - 1, right + margin)
        
        print(f"   检测到图标区域: ({left}, {top}) - ({right}, {bottom})")
        
        # 裁剪出图标
        icon_img = img.crop((left, top, right + 1, bottom + 1))
        icon_width, icon_height = icon_img.size
        
        print(f"   图标尺寸: {icon_width}x{icon_height}")
        
        # 分析图标，找到 V 字母和文字的分界线
        # 策略：从下往上扫描，找到文字区域
        
        icon_array = np.array(icon_img)
        icon_brightness = icon_array[:, :, :3].mean(axis=2)
        
        # 计算每一行的平均亮度
        row_brightness = icon_brightness.mean(axis=1)
        
        # 找到文字区域（底部亮度较高的区域）
        # 文字 "Visp" 是白色的，亮度会比较高
        text_threshold = row_brightness.max() * 0.7
        
        # 从底部往上找，找到第一个亮度低于阈值的行（V 字母的底部）
        v_bottom = icon_height - 1
        
        for y in range(icon_height - 1, icon_height // 2, -1):
            if row_brightness[y] < text_threshold:
                v_bottom = y
                break
        
        # 再往上找一点，确保完整包含 V 字母
        v_bottom = min(v_bottom + int(icon_height * 0.05), icon_height - 1)
        
        print(f"   V 字母底部位置: {v_bottom}")
        print(f"   去除文字区域: {v_bottom} - {icon_height}")
        
        # 裁剪掉底部文字，只保留 V 字母
        v_only = icon_img.crop((0, 0, icon_width, v_bottom))
        
        print(f"   V 字母尺寸: {v_only.size}")
        
        # 提取背景色（从图标中心区域采样）
        bg_sample_x = icon_width // 2
        bg_sample_y = icon_height // 2
        bg_color = icon_img.getpixel((bg_sample_x, bg_sample_y))
        
        print(f"   背景色: {bg_color}")
        
        # 创建正方形画布
        square_size = icon_width
        square_img = Image.new('RGBA', (square_size, square_size), bg_color)
        
        # 将 V 字母居中粘贴
        v_width, v_height = v_only.size
        paste_x = (square_size - v_width) // 2
        paste_y = (square_size - v_height) // 2
        
        square_img.paste(v_only, (paste_x, paste_y), v_only)
        
        print(f"   最终尺寸: {square_img.size}")
        
        return square_img
    else:
        print("   ⚠️  无法检测图标区域")
        return img

def create_rounded_mask(size, radius_ratio=0.225):
    """创建圆角矩形遮罩"""
    mask = Image.new('L', (size, size), 0)
    draw = ImageDraw.Draw(mask)
    radius = int(size * radius_ratio)
    draw.rounded_rectangle([(0, 0), (size - 1, size - 1)], radius=radius, fill=255)
    return mask

def generate_all_icons(base_icon, output_dir):
    """生成所有尺寸的图标"""
    os.makedirs(output_dir, exist_ok=True)
    
    print("\n🎨 生成应用图标...")
    
    # 应用图标尺寸
    app_sizes = [32, 128, 256, 512, 1024]
    
    for size in app_sizes:
        print(f"   生成 {size}x{size}")
        
        # 调整大小（使用高质量重采样）
        resized = base_icon.resize((size, size), Image.Resampling.LANCZOS)
        
        # 应用圆角遮罩
        mask = create_rounded_mask(size)
        
        # 创建最终图标
        final_icon = Image.new('RGBA', (size, size), (0, 0, 0, 0))
        final_icon.paste(resized, (0, 0), mask)
        
        # 保存
        if size == 1024:
            final_icon.save(f'{output_dir}/icon.png')
        
        final_icon.save(f'{output_dir}/{size}x{size}.png')
        
        # 保存 @2x 版本
        if size <= 256:
            final_icon.save(f'{output_dir}/{size}x{size}@2x.png')
    
    print("\n🔔 生成托盘图标...")
    
    # 托盘图标（简化版）
    tray_sizes = [22, 44]
    
    for size in tray_sizes:
        print(f"   生成托盘图标 {size}x{size}")
        
        # 调整大小
        tray_icon = base_icon.resize((size, size), Image.Resampling.LANCZOS)
        
        # 转换为灰度单色
        gray = tray_icon.convert('L')
        tray_rgba = Image.new('RGBA', (size, size), (0, 0, 0, 0))
        
        pixels = tray_rgba.load()
        gray_pixels = gray.load()
        
        for y in range(size):
            for x in range(size):
                alpha = gray_pixels[x, y]
                if alpha > 30:
                    pixels[x, y] = (60, 60, 60, alpha)
        
        if size == 22:
            tray_rgba.save(f'{output_dir}/icon_tray.png')
        
        tray_rgba.save(f'{output_dir}/icon_tray_{size}x{size}.png')
    
    # Windows ICO
    print("\n🪟 生成 Windows ICO...")
    ico_sizes = [(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    ico_images = []
    
    for size in ico_sizes:
        resized = base_icon.resize(size, Image.Resampling.LANCZOS)
        mask = create_rounded_mask(size[0])
        final = Image.new('RGBA', size, (0, 0, 0, 0))
        final.paste(resized, (0, 0), mask)
        ico_images.append(final)
    
    ico_images[0].save(
        f'{output_dir}/icon.ico',
        format='ICO',
        sizes=ico_sizes
    )
    print("   ✅ icon.ico")
    
    # macOS ICNS
    print("\n🍎 生成 macOS ICNS...")
    icns_sizes = [16, 32, 64, 128, 256, 512, 1024]
    
    iconset_dir = f'{output_dir}/icon.iconset'
    os.makedirs(iconset_dir, exist_ok=True)
    
    for size in icns_sizes:
        resized = base_icon.resize((size, size), Image.Resampling.LANCZOS)
        mask = create_rounded_mask(size)
        final = Image.new('RGBA', (size, size), (0, 0, 0, 0))
        final.paste(resized, (0, 0), mask)
        
        final.save(f'{iconset_dir}/icon_{size}x{size}.png')
        
        if size <= 512:
            size_2x = size * 2
            resized_2x = base_icon.resize((size_2x, size_2x), Image.Resampling.LANCZOS)
            mask_2x = create_rounded_mask(size_2x)
            final_2x = Image.new('RGBA', (size_2x, size_2x), (0, 0, 0, 0))
            final_2x.paste(resized_2x, (0, 0), mask_2x)
            final_2x.save(f'{iconset_dir}/icon_{size}x{size}@2x.png')
    
    # 使用 iconutil 生成 ICNS
    try:
        import subprocess
        subprocess.run([
            'iconutil', '-c', 'icns', iconset_dir,
            '-o', f'{output_dir}/icon.icns'
        ], check=True)
        print("   ✅ icon.icns")
        
        # 清理临时目录
        import shutil
        shutil.rmtree(iconset_dir)
    except (subprocess.CalledProcessError, FileNotFoundError):
        print("   ⚠️  iconutil 不可用（需要 macOS）")
    
    # Windows 托盘 ICO
    print("\n🪟 生成 Windows 托盘 ICO...")
    tray_ico_sizes = [(16, 16), (32, 32), (48, 48)]
    tray_ico_images = []
    
    for size in tray_ico_sizes:
        tray = base_icon.resize(size, Image.Resampling.LANCZOS)
        gray = tray.convert('L')
        tray_rgba = Image.new('RGBA', size, (0, 0, 0, 0))
        
        pixels = tray_rgba.load()
        gray_pixels = gray.load()
        
        for y in range(size[1]):
            for x in range(size[0]):
                alpha = gray_pixels[x, y]
                if alpha > 30:
                    pixels[x, y] = (60, 60, 60, alpha)
        
        tray_ico_images.append(tray_rgba)
    
    tray_ico_images[0].save(
        f'{output_dir}/icon_tray.ico',
        format='ICO',
        sizes=tray_ico_sizes
    )
    print("   ✅ icon_tray.ico")
    
    # Windows Store 图标
    print("\n🏪 生成 Windows Store 图标...")
    store_icon = base_icon.resize((310, 310), Image.Resampling.LANCZOS)
    mask = create_rounded_mask(310)
    final_store = Image.new('RGBA', (310, 310), (0, 0, 0, 0))
    final_store.paste(store_icon, (0, 0), mask)
    final_store.save(f'{output_dir}/Square310x310Logo.png')
    print("   ✅ Square310x310Logo.png")

def main():
    input_file = 'screen.png'
    output_dir = 'src-tauri/icons'
    
    print("🎨 从 Stitch 2.0 设计中提取 V 字母")
    print("=" * 50)
    print()
    
    # 检查输入文件
    if not os.path.exists(input_file):
        print(f"❌ 找不到文件: {input_file}")
        return
    
    # 提取 V 字母（去掉文字）
    v_icon = extract_icon_and_remove_text(input_file)
    
    # 生成所有尺寸
    generate_all_icons(v_icon, output_dir)
    
    print("\n" + "=" * 50)
    print("✨ 所有图标生成完成！")
    print(f"📁 输出目录: {output_dir}")
    print("\n图标特点:")
    print("  • 使用 Stitch 2.0 专业设计的 V 字母")
    print("  • 流线型弧度，高端现代")
    print("  • 青色到粉紫色渐变")
    print("  • 3D 立体效果")
    print("  • 深色背景")
    print("  • 去除了底部文字，只保留 V")

if __name__ == '__main__':
    # 检查依赖
    try:
        import numpy
    except ImportError:
        print("⚠️  需要安装 numpy")
        print("运行: pip3 install numpy")
        exit(1)
    
    main()
