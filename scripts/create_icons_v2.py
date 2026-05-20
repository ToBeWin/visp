#!/usr/bin/env python3
"""
Visp Icon Generator V2 - 优雅现代设计
创建应用图标和托盘图标
"""

from PIL import Image, ImageDraw, ImageFont
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

def draw_v_letter(draw, size, color, thickness):
    """绘制优雅的 V 字母"""
    center_x = size // 2
    top_y = size * 0.3
    bottom_y = size * 0.75
    width = size * 0.35
    
    # 左边的线
    left_top = (center_x - width, top_y)
    left_bottom = (center_x, bottom_y)
    
    # 右边的线
    right_top = (center_x + width, top_y)
    right_bottom = (center_x, bottom_y)
    
    # 绘制 V 字母（使用多条线创建粗细效果）
    for i in range(thickness):
        offset = i - thickness // 2
        draw.line([
            (left_top[0] + offset * 0.5, left_top[1]),
            (left_bottom[0] + offset, left_bottom[1])
        ], fill=color, width=2)
        
        draw.line([
            (right_top[0] - offset * 0.5, right_top[1]),
            (right_bottom[0] + offset, right_bottom[1])
        ], fill=color, width=2)

def draw_sound_waves(draw, size, color, alpha=180):
    """绘制声波效果"""
    center_x = size // 2
    center_y = size // 2
    
    # 绘制三个同心圆弧（声波）
    wave_color = (*color[:3], alpha)
    
    for i in range(3):
        radius = size * (0.25 + i * 0.08)
        thickness = max(2, size // 80)
        
        # 左侧声波
        bbox_left = [
            center_x - radius * 2, center_y - radius,
            center_x - radius * 0.5, center_y + radius
        ]
        draw.arc(bbox_left, start=-30, end=30, fill=wave_color, width=thickness)
        
        # 右侧声波
        bbox_right = [
            center_x + radius * 0.5, center_y - radius,
            center_x + radius * 2, center_y + radius
        ]
        draw.arc(bbox_right, start=150, end=210, fill=wave_color, width=thickness)

def create_app_icon(size):
    """创建应用图标 - 现代优雅设计"""
    # 创建渐变背景（深蓝到紫色）
    color1 = (59, 130, 246)   # 蓝色 #3B82F6
    color2 = (139, 92, 246)   # 紫色 #8B5CF6
    
    image = create_gradient_background(size, color1, color2)
    
    # 添加圆角矩形遮罩
    radius = int(size * 0.225)  # 22.5% 圆角
    mask = create_rounded_square(size, radius, (255, 255, 255, 255))
    
    # 应用遮罩
    result = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    result.paste(image, (0, 0), mask)
    
    # 在图标上绘制内容
    draw = ImageDraw.Draw(result)
    
    # 绘制声波（半透明）
    draw_sound_waves(draw, size, (255, 255, 255), alpha=100)
    
    # 绘制 V 字母（白色，粗体）
    thickness = max(8, size // 16)
    draw_v_letter(draw, size, (255, 255, 255, 255), thickness)
    
    # 添加微妙的光泽效果（顶部高光）
    overlay = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    overlay_draw = ImageDraw.Draw(overlay)
    
    # 顶部渐变高光
    for y in range(size // 3):
        alpha = int(40 * (1 - y / (size // 3)))
        overlay_draw.line([(0, y), (size, y)], fill=(255, 255, 255, alpha))
    
    # 应用高光
    result = Image.alpha_composite(result, overlay)
    
    return result

def create_tray_icon(size):
    """创建托盘图标 - 简约单色设计"""
    image = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    
    # 使用深灰色（适合亮色和暗色主题）
    color = (60, 60, 60, 255)
    
    # 绘制简化的 V 字母
    thickness = max(2, size // 8)
    draw_v_letter(draw, size, color, thickness)
    
    # 添加简化的声波（只有一个）
    center_x = size // 2
    center_y = size // 2
    radius = size * 0.3
    thickness_wave = max(1, size // 16)
    
    # 左侧声波
    bbox_left = [
        center_x - radius * 1.8, center_y - radius * 0.8,
        center_x - radius * 0.6, center_y + radius * 0.8
    ]
    draw.arc(bbox_left, start=-25, end=25, fill=color, width=thickness_wave)
    
    # 右侧声波
    bbox_right = [
        center_x + radius * 0.6, center_y - radius * 0.8,
        center_x + radius * 1.8, center_y + radius * 0.8
    ]
    draw.arc(bbox_right, start=155, end=205, fill=color, width=thickness_wave)
    
    return image

def main():
    """生成所有图标"""
    import os
    
    # 图标输出目录
    output_dir = 'src-tauri/icons'
    os.makedirs(output_dir, exist_ok=True)
    
    print("🎨 生成 Visp 图标 V2 - 优雅现代设计")
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
    
    # Windows ICO（多尺寸）
    print("\n🪟 生成 Windows ICO")
    ico_sizes = [(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    ico_images = [create_app_icon(size[0]) for size in ico_sizes]
    ico_images[0].save(
        f'{output_dir}/icon.ico',
        format='ICO',
        sizes=ico_sizes
    )
    print("   ✅ 保存: icon.ico")
    
    # macOS ICNS（需要多个尺寸）
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
    
    # 使用 iconutil 生成 ICNS（仅在 macOS 上）
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
        print("   💡 请在 macOS 上运行此脚本以生成 .icns 文件")
    
    # 托盘图标
    print("\n🔔 生成托盘图标")
    tray_sizes = [22, 44]
    
    for size in tray_sizes:
        print(f"   生成托盘图标: {size}x{size}")
        tray_icon = create_tray_icon(size)
        
        # 保存为 Template（macOS 托盘图标命名约定）
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
    print("  • 现代渐变背景（蓝色到紫色）")
    print("  • 优雅的 V 字母设计")
    print("  • 声波效果增强科技感")
    print("  • 圆角矩形符合现代设计")
    print("  • 托盘图标简约单色")

if __name__ == '__main__':
    main()
