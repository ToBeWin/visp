#!/usr/bin/env python3
"""
Visp Icon Generator V3 - 极简设计
简洁、现代、易识别
"""

from PIL import Image, ImageDraw
import math

def create_gradient_background(size, color1, color2):
    """创建渐变背景"""
    image = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    
    for y in range(size):
        ratio = y / size
        r = int(color1[0] * (1 - ratio) + color2[0] * ratio)
        g = int(color1[1] * (1 - ratio) + color2[1] * ratio)
        b = int(color1[2] * (1 - ratio) + color2[2] * ratio)
        draw.line([(0, y), (size, y)], fill=(r, g, b, 255))
    
    return image

def create_rounded_square(size, radius, color):
    """创建圆角矩形"""
    image = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    draw.rounded_rectangle([(0, 0), (size-1, size-1)], radius=radius, fill=color)
    return image

def draw_simple_v(draw, size, color, thickness):
    """绘制简洁的 V 字母 - 极简风格"""
    center_x = size // 2
    top_y = size * 0.35
    bottom_y = size * 0.7
    width = size * 0.28
    
    # V 字母的两条线
    left_top = (center_x - width, top_y)
    bottom = (center_x, bottom_y)
    right_top = (center_x + width, top_y)
    
    # 绘制左边
    draw.line([left_top, bottom], fill=color, width=thickness)
    
    # 绘制右边
    draw.line([right_top, bottom], fill=color, width=thickness)

def create_app_icon(size):
    """创建应用图标 - 极简设计"""
    # 创建渐变背景（柔和的蓝紫渐变）
    color1 = (99, 102, 241)   # 靛蓝 #6366F1
    color2 = (168, 85, 247)   # 紫色 #A855F7
    
    image = create_gradient_background(size, color1, color2)
    
    # 添加圆角矩形遮罩
    radius = int(size * 0.225)
    mask = create_rounded_square(size, radius, (255, 255, 255, 255))
    
    # 应用遮罩
    result = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    result.paste(image, (0, 0), mask)
    
    # 在图标上绘制 V 字母
    draw = ImageDraw.Draw(result)
    
    # 绘制简洁的 V 字母（白色，适中粗细）
    thickness = max(int(size * 0.08), 4)  # 8% 的粗细
    draw_simple_v(draw, size, (255, 255, 255, 255), thickness)
    
    return result

def create_tray_icon(size):
    """创建托盘图标 - 极简单色"""
    image = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    
    # 深灰色
    color = (60, 60, 60, 255)
    
    # 绘制简洁的 V 字母
    thickness = max(int(size * 0.12), 2)
    draw_simple_v(draw, size, color, thickness)
    
    return image

def main():
    """生成所有图标"""
    import os
    
    # 图标输出目录
    output_dir = 'src-tauri/icons'
    os.makedirs(output_dir, exist_ok=True)
    
    print("🎨 生成 Visp 图标 V3 - 极简设计")
    print("=" * 50)
    
    # 应用图标尺寸
    app_sizes = [32, 128, 256, 512, 1024]
    
    for size in app_sizes:
        print(f"📱 生成应用图标: {size}x{size}")
        icon = create_app_icon(size)
        
        # 保存 PNG
        if size == 1024:
            icon.save(f'{output_dir}/icon.png')
            print(f"   ✅ 保存: icon.png")
        
        icon.save(f'{output_dir}/{size}x{size}.png')
        print(f"   ✅ 保存: {size}x{size}.png")
        
        # 保存 @2x 版本
        if size <= 256:
            icon.save(f'{output_dir}/{size}x{size}@2x.png')
            print(f"   ✅ 保存: {size}x{size}@2x.png")
    
    # Windows ICO
    print("\n🪟 生成 Windows ICO")
    ico_sizes = [(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    ico_images = [create_app_icon(size[0]) for size in ico_sizes]
    ico_images[0].save(
        f'{output_dir}/icon.ico',
        format='ICO',
        sizes=ico_sizes
    )
    print("   ✅ 保存: icon.ico")
    
    # macOS ICNS
    print("\n🍎 生成 macOS ICNS")
    icns_sizes = [16, 32, 64, 128, 256, 512, 1024]
    
    # 创建临时 iconset 目录
    iconset_dir = f'{output_dir}/icon.iconset'
    os.makedirs(iconset_dir, exist_ok=True)
    
    for size in icns_sizes:
        icon = create_app_icon(size)
        icon.save(f'{iconset_dir}/icon_{size}x{size}.png')
        
        if size <= 512:
            icon_2x = create_app_icon(size * 2)
            icon_2x.save(f'{iconset_dir}/icon_{size}x{size}@2x.png')
    
    # 使用 iconutil 生成 ICNS
    try:
        import subprocess
        subprocess.run([
            'iconutil', '-c', 'icns', iconset_dir,
            '-o', f'{output_dir}/icon.icns'
        ], check=True)
        print("   ✅ 保存: icon.icns")
        
        # 清理临时目录
        import shutil
        shutil.rmtree(iconset_dir)
    except (subprocess.CalledProcessError, FileNotFoundError):
        print("   ⚠️  iconutil 不可用（需要 macOS）")
    
    # 托盘图标
    print("\n🔔 生成托盘图标")
    tray_sizes = [22, 44]
    
    for size in tray_sizes:
        print(f"   生成托盘图标: {size}x{size}")
        tray_icon = create_tray_icon(size)
        
        if size == 22:
            tray_icon.save(f'{output_dir}/icon_tray.png')
            print(f"   ✅ 保存: icon_tray.png")
        
        tray_icon.save(f'{output_dir}/icon_tray_{size}x{size}.png')
        print(f"   ✅ 保存: icon_tray_{size}x{size}.png")
    
    # Windows 托盘图标
    print("\n🪟 生成 Windows 托盘 ICO")
    tray_ico_sizes = [(16, 16), (32, 32), (48, 48)]
    tray_ico_images = [create_tray_icon(size[0]) for size in tray_ico_sizes]
    tray_ico_images[0].save(
        f'{output_dir}/icon_tray.ico',
        format='ICO',
        sizes=tray_ico_sizes
    )
    print("   ✅ 保存: icon_tray.ico")
    
    # Windows Store 图标
    print("\n🏪 生成 Windows Store 图标")
    store_icon = create_app_icon(310)
    store_icon.save(f'{output_dir}/Square310x310Logo.png')
    print("   ✅ 保存: Square310x310Logo.png")
    
    print("\n" + "=" * 50)
    print("✨ 所有图标生成完成！")
    print(f"📁 输出目录: {output_dir}")
    print("\n图标特点:")
    print("  • 极简设计，只保留核心元素")
    print("  • 柔和的靛蓝到紫色渐变")
    print("  • 简洁的 V 字母，无多余装饰")
    print("  • 圆角矩形，现代感强")
    print("  • 托盘图标极简单色")
    print("  • 易于识别，视觉清爽")

if __name__ == '__main__':
    main()
