#!/usr/bin/env python3
"""
处理 Stitch 2.0 设计的图标
去掉底部的 "Visp" 文字，保留 V 字母
"""

from PIL import Image, ImageDraw
import os

def crop_icon_remove_text(input_path, output_dir):
    """
    从截图中提取中间的圆角矩形图标，并去掉底部的文字
    """
    print(f"📖 读取图标: {input_path}")
    img = Image.open(input_path)
    
    # 转换为 RGBA
    if img.mode != 'RGBA':
        img = img.convert('RGBA')
    
    width, height = img.size
    print(f"   原始尺寸: {width}x{height}")
    
    import numpy as np
    img_array = np.array(img)
    
    # 策略：找到图标的圆角矩形边界
    # 图标应该是截图中最亮的矩形区域
    
    # 计算每个像素的亮度
    brightness = img_array[:, :, :3].mean(axis=2)
    
    # 找到亮度较高的区域（图标区域）
    # 使用更高的阈值来找到图标本身
    threshold = brightness.mean() + brightness.std() * 0.5
    
    # 创建二值化图像
    binary = brightness > threshold
    
    # 找到所有亮区域的边界
    rows_with_icon = np.where(binary.any(axis=1))[0]
    cols_with_icon = np.where(binary.any(axis=0))[0]
    
    if len(rows_with_icon) > 0 and len(cols_with_icon) > 0:
        # 获取边界
        top = rows_with_icon[0]
        bottom = rows_with_icon[-1]
        left = cols_with_icon[0]
        right = cols_with_icon[-1]
        
        # 添加一些边距以确保完整提取
        margin = 10
        top = max(0, top - margin)
        bottom = min(height - 1, bottom + margin)
        left = max(0, left - margin)
        right = min(width - 1, right + margin)
        
        print(f"   检测到图标区域: ({left}, {top}) - ({right}, {bottom})")
        
        # 裁剪出图标区域
        icon_img = img.crop((left, top, right + 1, bottom + 1))
        icon_width, icon_height = icon_img.size
        
        print(f"   图标尺寸: {icon_width}x{icon_height}")
        
        # 去掉底部的 "Visp" 文字
        # 文字大约占图标高度的 18-22%
        text_height_ratio = 0.20  # 文字区域占比
        crop_bottom = int(icon_height * (1 - text_height_ratio))
        
        # 裁剪掉底部文字
        icon_no_text = icon_img.crop((0, 0, icon_width, crop_bottom))
        
        print(f"   去除文字后尺寸: {icon_no_text.size}")
        
        # 提取背景色（从图标中间区域采样）
        bg_sample_x = icon_width // 2
        bg_sample_y = icon_height // 2
        bg_color = icon_img.getpixel((bg_sample_x, bg_sample_y))
        
        print(f"   背景色: {bg_color}")
        
        # 创建正方形画布（使用图标宽度）
        square_size = icon_width
        square_img = Image.new('RGBA', (square_size, square_size), bg_color)
        
        # 将去掉文字的图标居中粘贴
        v_width, v_height = icon_no_text.size
        paste_x = (square_size - v_width) // 2
        paste_y = (square_size - v_height) // 2
        
        square_img.paste(icon_no_text, (paste_x, paste_y), icon_no_text)
        
        print(f"   最终尺寸: {square_img.size}")
        
        return square_img
    else:
        print("   ⚠️  无法检测图标区域")
        # 尝试从中心区域提取
        center_x = width // 2
        center_y = height // 2
        icon_size = min(width, height) // 2
        
        left = center_x - icon_size // 2
        right = center_x + icon_size // 2
        top = center_y - icon_size // 2
        bottom = center_y + icon_size // 2
        
        return img.crop((left, top, right, bottom))

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
        
        # 调整大小
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
    
    # 托盘图标（简化版，只保留 V 的轮廓）
    tray_sizes = [22, 44]
    
    for size in tray_sizes:
        print(f"   生成托盘图标 {size}x{size}")
        
        # 调整大小
        tray_icon = base_icon.resize((size, size), Image.Resampling.LANCZOS)
        
        # 转换为灰度，然后转为深灰色
        gray = tray_icon.convert('L')
        tray_rgba = Image.new('RGBA', (size, size), (0, 0, 0, 0))
        
        # 使用灰度作为 alpha 通道，颜色设为深灰
        pixels = tray_rgba.load()
        gray_pixels = gray.load()
        
        for y in range(size):
            for x in range(size):
                alpha = gray_pixels[x, y]
                if alpha > 30:  # 只保留明显的部分
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
    
    print("🎨 处理 Stitch 2.0 设计的图标")
    print("=" * 50)
    print()
    
    # 检查输入文件
    if not os.path.exists(input_file):
        print(f"❌ 找不到文件: {input_file}")
        return
    
    # 处理图标，去掉文字
    base_icon = crop_icon_remove_text(input_file, output_dir)
    
    # 生成所有尺寸
    generate_all_icons(base_icon, output_dir)
    
    print("\n" + "=" * 50)
    print("✨ 所有图标生成完成！")
    print(f"📁 输出目录: {output_dir}")
    print("\n图标特点:")
    print("  • 保留了 Stitch 2.0 设计的精美 V 字母")
    print("  • 去除了底部的 'Visp' 文字")
    print("  • 蓝紫渐变，立体感强")
    print("  • 深色背景，现代专业")
    print("  • 圆角矩形，符合系统设计语言")

if __name__ == '__main__':
    # 检查依赖
    try:
        import numpy
    except ImportError:
        print("⚠️  需要安装 numpy")
        print("运行: pip3 install numpy")
        exit(1)
    
    main()
