#!/usr/bin/env python3
"""
Visp Icon Generator V4 - Stitch 2.0 风格
模仿 Stitch 2.0 设计的 V 字母效果：
- 蓝紫渐变（青色到粉紫色）
- 3D 立体感
- 柔和的阴影
- 深色背景
"""

from PIL import Image, ImageDraw, ImageFilter
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

def draw_3d_v_letter(size, bg_color):
    """
    绘制 3D 效果的 V 字母 - Stitch 2.0 风格
    特点：
    - 蓝紫渐变（左边青色，右边粉紫色）
    - 立体感（阴影层）
    - 柔和的边缘
    - 弯曲的 V 形状
    """
    # 创建画布
    canvas = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    
    # V 字母的基本参数
    center_x = size // 2
    top_y = int(size * 0.30)      # V 的顶部
    bottom_y = int(size * 0.72)   # V 的底部
    width = int(size * 0.35)      # V 的宽度
    thickness = int(size * 0.12)  # V 的粗细
    
    # 左右两个端点
    left_top = (center_x - width, top_y)
    right_top = (center_x + width, top_y)
    bottom = (center_x, bottom_y)
    
    # === 第一步：绘制阴影层（深色，模糊） ===
    shadow_layer = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    shadow_draw = ImageDraw.Draw(shadow_layer)
    
    # 阴影偏移
    shadow_offset = int(size * 0.015)
    
    # 左边阴影
    shadow_left_points = [
        (left_top[0] + shadow_offset, left_top[1] + shadow_offset),
        (left_top[0] + thickness + shadow_offset, left_top[1] + shadow_offset),
        (bottom[0] + thickness // 2 + shadow_offset, bottom[1] + shadow_offset),
        (bottom[0] - thickness // 2 + shadow_offset, bottom[1] + shadow_offset),
    ]
    shadow_draw.polygon(shadow_left_points, fill=(0, 0, 0, 80))
    
    # 右边阴影
    shadow_right_points = [
        (right_top[0] + shadow_offset, right_top[1] + shadow_offset),
        (right_top[0] - thickness + shadow_offset, right_top[1] + shadow_offset),
        (bottom[0] - thickness // 2 + shadow_offset, bottom[1] + shadow_offset),
        (bottom[0] + thickness // 2 + shadow_offset, bottom[1] + shadow_offset),
    ]
    shadow_draw.polygon(shadow_right_points, fill=(0, 0, 0, 80))
    
    # 模糊阴影
    shadow_layer = shadow_layer.filter(ImageFilter.GaussianBlur(radius=size * 0.01))
    
    # 将阴影合并到画布
    canvas = Image.alpha_composite(canvas, shadow_layer)
    
    # === 第二步：绘制 V 字母主体（渐变） ===
    
    # 创建左边部分（青色渐变）
    left_part = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    left_draw = ImageDraw.Draw(left_part)
    
    # 左边的四个顶点（梯形）
    left_points = [
        left_top,
        (left_top[0] + thickness, left_top[1]),
        (bottom[0] + thickness // 2, bottom[1]),
        (bottom[0] - thickness // 2, bottom[1]),
    ]
    
    # 绘制左边部分的渐变
    # 从青色 (#4FC3F7) 到蓝紫色 (#7E57C2)
    for i in range(len(left_points) - 1):
        # 计算这一段的渐变
        y_start = min(left_points[i][1], left_points[i+1][1])
        y_end = max(left_points[i][1], left_points[i+1][1])
        
        for y in range(y_start, y_end + 1):
            ratio = (y - top_y) / (bottom_y - top_y)
            
            # 青色到蓝紫色渐变
            r = int(79 * (1 - ratio) + 126 * ratio)
            g = int(195 * (1 - ratio) + 87 * ratio)
            b = int(247 * (1 - ratio) + 194 * ratio)
            
            # 计算这一行的左右边界
            x_left = left_top[0] + (bottom[0] - thickness // 2 - left_top[0]) * (y - top_y) / (bottom_y - top_y)
            x_right = (left_top[0] + thickness) + (bottom[0] + thickness // 2 - left_top[0] - thickness) * (y - top_y) / (bottom_y - top_y)
            
            if x_right > x_left:
                left_draw.line([(int(x_left), y), (int(x_right), y)], fill=(r, g, b, 255), width=1)
    
    # 创建右边部分（粉紫色渐变）
    right_part = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    right_draw = ImageDraw.Draw(right_part)
    
    # 右边的四个顶点
    right_points = [
        right_top,
        (right_top[0] - thickness, right_top[1]),
        (bottom[0] - thickness // 2, bottom[1]),
        (bottom[0] + thickness // 2, bottom[1]),
    ]
    
    # 绘制右边部分的渐变
    # 从蓝紫色 (#7E57C2) 到粉紫色 (#EC407A)
    for y in range(top_y, bottom_y + 1):
        ratio = (y - top_y) / (bottom_y - top_y)
        
        # 蓝紫色到粉紫色渐变
        r = int(126 * (1 - ratio) + 236 * ratio)
        g = int(87 * (1 - ratio) + 64 * ratio)
        b = int(194 * (1 - ratio) + 122 * ratio)
        
        # 计算这一行的左右边界
        x_left = (right_top[0] - thickness) + (bottom[0] - thickness // 2 - right_top[0] + thickness) * (y - top_y) / (bottom_y - top_y)
        x_right = right_top[0] + (bottom[0] + thickness // 2 - right_top[0]) * (y - top_y) / (bottom_y - top_y)
        
        if x_right > x_left:
            right_draw.line([(int(x_left), y), (int(x_right), y)], fill=(r, g, b, 255), width=1)
    
    # 合并左右两部分
    canvas = Image.alpha_composite(canvas, left_part)
    canvas = Image.alpha_composite(canvas, right_part)
    
    # === 第三步：添加高光效果 ===
    highlight = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    highlight_draw = ImageDraw.Draw(highlight)
    
    # 在 V 字母顶部添加柔和的高光
    highlight_thickness = thickness // 3
    
    # 左边高光
    left_highlight_points = [
        left_top,
        (left_top[0] + thickness, left_top[1]),
        (left_top[0] + thickness, left_top[1] + highlight_thickness),
        (left_top[0], left_top[1] + highlight_thickness),
    ]
    highlight_draw.polygon(left_highlight_points, fill=(255, 255, 255, 40))
    
    # 右边高光
    right_highlight_points = [
        right_top,
        (right_top[0] - thickness, right_top[1]),
        (right_top[0] - thickness, right_top[1] + highlight_thickness),
        (right_top[0], right_top[1] + highlight_thickness),
    ]
    highlight_draw.polygon(right_highlight_points, fill=(255, 255, 255, 40))
    
    # 模糊高光
    highlight = highlight.filter(ImageFilter.GaussianBlur(radius=size * 0.008))
    
    # 合并高光
    canvas = Image.alpha_composite(canvas, highlight)
    
    return canvas

def create_app_icon(size):
    """创建应用图标 - Stitch 2.0 风格"""
    # 深色背景（深灰蓝色）
    bg_color = (30, 34, 38, 255)  # 类似 Stitch 2.0 的深色背景
    
    # 创建圆角矩形背景
    radius = int(size * 0.225)
    background = create_rounded_square(size, radius, bg_color)
    
    # 绘制 3D V 字母
    v_letter = draw_3d_v_letter(size, bg_color)
    
    # 合并
    result = Image.alpha_composite(background, v_letter)
    
    return result

def create_tray_icon(size):
    """创建托盘图标 - 简化版"""
    image = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    
    # 深灰色
    color = (60, 60, 60, 255)
    
    # 简化的 V 字母
    center_x = size // 2
    top_y = int(size * 0.25)
    bottom_y = int(size * 0.75)
    width = int(size * 0.35)
    thickness = max(int(size * 0.15), 2)
    
    left_top = (center_x - width, top_y)
    right_top = (center_x + width, top_y)
    bottom = (center_x, bottom_y)
    
    # 绘制左边
    draw.line([left_top, bottom], fill=color, width=thickness)
    
    # 绘制右边
    draw.line([right_top, bottom], fill=color, width=thickness)
    
    return image

def main():
    """生成所有图标"""
    import os
    
    # 图标输出目录
    output_dir = 'src-tauri/icons'
    os.makedirs(output_dir, exist_ok=True)
    
    print("🎨 生成 Visp 图标 V4 - Stitch 2.0 风格")
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
    print("  • Stitch 2.0 风格的 3D V 字母")
    print("  • 青色到粉紫色的渐变效果")
    print("  • 立体阴影，柔和高光")
    print("  • 深色背景，现代专业")
    print("  • 圆角矩形，符合系统设计语言")

if __name__ == '__main__':
    main()
