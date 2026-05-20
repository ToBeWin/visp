#!/usr/bin/env python3
"""
Visp Icon Generator V6 - 更亮的背景
基于 V5 的流线型设计，但使用更亮的背景色
"""

from PIL import Image, ImageDraw, ImageFilter
import math

def create_rounded_square(size, radius, color):
    """创建圆角矩形"""
    image = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    draw.rounded_rectangle([(0, 0), (size-1, size-1)], radius=radius, fill=color)
    return image

def bezier_curve(p0, p1, p2, p3, num_points=100):
    """三次贝塞尔曲线"""
    points = []
    for i in range(num_points + 1):
        t = i / num_points
        x = (1-t)**3 * p0[0] + 3*(1-t)**2*t * p1[0] + 3*(1-t)*t**2 * p2[0] + t**3 * p3[0]
        y = (1-t)**3 * p0[1] + 3*(1-t)**2*t * p1[1] + 3*(1-t)*t**2 * p2[1] + t**3 * p3[1]
        points.append((x, y))
    return points

def draw_curved_v_letter(size):
    """绘制流线型弧度 V 字母"""
    scale = 4
    canvas_size = size * scale
    canvas = Image.new('RGBA', (canvas_size, canvas_size), (0, 0, 0, 0))
    
    center_x = canvas_size // 2
    top_y = int(canvas_size * 0.28)
    bottom_y = int(canvas_size * 0.75)
    top_width = int(canvas_size * 0.38)
    thickness = int(canvas_size * 0.11)
    
    # 左边外侧曲线
    left_outer_start = (center_x - top_width, top_y)
    left_outer_end = (center_x, bottom_y)
    left_outer_ctrl1 = (center_x - top_width * 0.8, top_y + (bottom_y - top_y) * 0.3)
    left_outer_ctrl2 = (center_x - top_width * 0.3, top_y + (bottom_y - top_y) * 0.7)
    left_outer_curve = bezier_curve(left_outer_start, left_outer_ctrl1, left_outer_ctrl2, left_outer_end, num_points=200)
    
    # 左边内侧曲线
    left_inner_start = (center_x - top_width + thickness, top_y)
    left_inner_end = (center_x + thickness // 2, bottom_y)
    left_inner_ctrl1 = (center_x - top_width * 0.8 + thickness, top_y + (bottom_y - top_y) * 0.3)
    left_inner_ctrl2 = (center_x - top_width * 0.3 + thickness, top_y + (bottom_y - top_y) * 0.7)
    left_inner_curve = bezier_curve(left_inner_start, left_inner_ctrl1, left_inner_ctrl2, left_inner_end, num_points=200)
    
    # 右边外侧曲线
    right_outer_start = (center_x + top_width, top_y)
    right_outer_end = (center_x, bottom_y)
    right_outer_ctrl1 = (center_x + top_width * 0.8, top_y + (bottom_y - top_y) * 0.3)
    right_outer_ctrl2 = (center_x + top_width * 0.3, top_y + (bottom_y - top_y) * 0.7)
    right_outer_curve = bezier_curve(right_outer_start, right_outer_ctrl1, right_outer_ctrl2, right_outer_end, num_points=200)
    
    # 右边内侧曲线
    right_inner_start = (center_x + top_width - thickness, top_y)
    right_inner_end = (center_x - thickness // 2, bottom_y)
    right_inner_ctrl1 = (center_x + top_width * 0.8 - thickness, top_y + (bottom_y - top_y) * 0.3)
    right_inner_ctrl2 = (center_x + top_width * 0.3 - thickness, top_y + (bottom_y - top_y) * 0.7)
    right_inner_curve = bezier_curve(right_inner_start, right_inner_ctrl1, right_inner_ctrl2, right_inner_end, num_points=200)
    
    # 左边部分：青色到蓝紫色渐变
    left_part = Image.new('RGBA', (canvas_size, canvas_size), (0, 0, 0, 0))
    left_draw = ImageDraw.Draw(left_part)
    
    for y in range(top_y, bottom_y + 1):
        outer_x = None
        inner_x = None
        
        for i, (x, py) in enumerate(left_outer_curve):
            if int(py) == y:
                outer_x = x
                break
        
        for i, (x, py) in enumerate(left_inner_curve):
            if int(py) == y:
                inner_x = x
                break
        
        if outer_x is not None and inner_x is not None:
            ratio = (y - top_y) / (bottom_y - top_y)
            # 更亮的青色到蓝紫色渐变
            # 从亮青色 #5DD5F7 到亮蓝紫色 #9E77ED
            r = int(93 * (1 - ratio) + 158 * ratio)
            g = int(213 * (1 - ratio) + 119 * ratio)
            b = int(247 * (1 - ratio) + 237 * ratio)
            left_draw.line([(int(outer_x), y), (int(inner_x), y)], fill=(r, g, b, 255), width=1)
    
    # 右边部分：蓝紫色到粉紫色渐变
    right_part = Image.new('RGBA', (canvas_size, canvas_size), (0, 0, 0, 0))
    right_draw = ImageDraw.Draw(right_part)
    
    for y in range(top_y, bottom_y + 1):
        outer_x = None
        inner_x = None
        
        for i, (x, py) in enumerate(right_outer_curve):
            if int(py) == y:
                outer_x = x
                break
        
        for i, (x, py) in enumerate(right_inner_curve):
            if int(py) == y:
                inner_x = x
                break
        
        if outer_x is not None and inner_x is not None:
            ratio = (y - top_y) / (bottom_y - top_y)
            # 更亮的蓝紫色到粉紫色渐变
            # 从亮蓝紫色 #9E77ED 到亮粉紫色 #F472B6
            r = int(158 * (1 - ratio) + 244 * ratio)
            g = int(119 * (1 - ratio) + 114 * ratio)
            b = int(237 * (1 - ratio) + 182 * ratio)
            right_draw.line([(int(inner_x), y), (int(outer_x), y)], fill=(r, g, b, 255), width=1)
    
    canvas = Image.alpha_composite(canvas, left_part)
    canvas = Image.alpha_composite(canvas, right_part)
    
    # 添加阴影
    shadow = Image.new('RGBA', (canvas_size, canvas_size), (0, 0, 0, 0))
    shadow_draw = ImageDraw.Draw(shadow)
    
    shadow_offset = int(canvas_size * 0.012)
    left_polygon = left_outer_curve + list(reversed(left_inner_curve))
    left_shadow_polygon = [(x + shadow_offset, y + shadow_offset) for x, y in left_polygon]
    shadow_draw.polygon(left_shadow_polygon, fill=(0, 0, 0, 60))
    
    right_polygon = right_outer_curve + list(reversed(right_inner_curve))
    right_shadow_polygon = [(x + shadow_offset, y + shadow_offset) for x, y in right_polygon]
    shadow_draw.polygon(right_shadow_polygon, fill=(0, 0, 0, 60))
    
    shadow = shadow.filter(ImageFilter.GaussianBlur(radius=canvas_size * 0.008))
    
    final = Image.new('RGBA', (canvas_size, canvas_size), (0, 0, 0, 0))
    final = Image.alpha_composite(final, shadow)
    final = Image.alpha_composite(final, canvas)
    
    # 添加高光
    highlight = Image.new('RGBA', (canvas_size, canvas_size), (0, 0, 0, 0))
    highlight_draw = ImageDraw.Draw(highlight)
    
    highlight_height = int(canvas_size * 0.08)
    
    for i in range(len(left_outer_curve) // 4):
        x1, y1 = left_outer_curve[i]
        x2, y2 = left_inner_curve[i]
        
        if y1 < top_y + highlight_height:
            alpha = int(50 * (1 - (y1 - top_y) / highlight_height))
            highlight_draw.line([(int(x1), int(y1)), (int(x2), int(y2))], fill=(255, 255, 255, alpha), width=2)
    
    for i in range(len(right_outer_curve) // 4):
        x1, y1 = right_outer_curve[i]
        x2, y2 = right_inner_curve[i]
        
        if y1 < top_y + highlight_height:
            alpha = int(50 * (1 - (y1 - top_y) / highlight_height))
            highlight_draw.line([(int(x1), int(y1)), (int(x2), int(y2))], fill=(255, 255, 255, alpha), width=2)
    
    highlight = highlight.filter(ImageFilter.GaussianBlur(radius=canvas_size * 0.006))
    final = Image.alpha_composite(final, highlight)
    
    final = final.resize((size, size), Image.Resampling.LANCZOS)
    
    return final

def create_app_icon(size):
    """创建应用图标 - 白色背景"""
    # 纯白色背景
    bg_color = (255, 255, 255, 255)
    
    radius = int(size * 0.225)
    background = create_rounded_square(size, radius, bg_color)
    
    v_letter = draw_curved_v_letter(size)
    
    result = Image.alpha_composite(background, v_letter)
    
    return result

def create_tray_icon(size):
    """创建托盘图标"""
    image = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    
    color = (60, 60, 60, 255)
    
    center_x = size // 2
    top_y = int(size * 0.25)
    bottom_y = int(size * 0.75)
    width = int(size * 0.35)
    thickness = max(int(size * 0.15), 2)
    
    left_top = (center_x - width, top_y)
    right_top = (center_x + width, top_y)
    bottom = (center_x, bottom_y)
    
    draw.line([left_top, bottom], fill=color, width=thickness)
    draw.line([right_top, bottom], fill=color, width=thickness)
    
    return image

def main():
    """生成所有图标"""
    import os
    
    output_dir = 'src-tauri/icons'
    os.makedirs(output_dir, exist_ok=True)
    
    print("🎨 生成 Visp 图标 V6 - 更亮的背景")
    print("=" * 50)
    
    app_sizes = [32, 128, 256, 512, 1024]
    
    for size in app_sizes:
        print(f"📱 生成应用图标: {size}x{size}")
        icon = create_app_icon(size)
        
        if size == 1024:
            icon.save(f'{output_dir}/icon.png')
            print(f"   ✅ 保存: icon.png")
        
        icon.save(f'{output_dir}/{size}x{size}.png')
        print(f"   ✅ 保存: {size}x{size}.png")
        
        if size <= 256:
            icon.save(f'{output_dir}/{size}x{size}@2x.png')
            print(f"   ✅ 保存: {size}x{size}@2x.png")
    
    print("\n🪟 生成 Windows ICO")
    ico_sizes = [(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    ico_images = [create_app_icon(size[0]) for size in ico_sizes]
    ico_images[0].save(f'{output_dir}/icon.ico', format='ICO', sizes=ico_sizes)
    print("   ✅ 保存: icon.ico")
    
    print("\n🍎 生成 macOS ICNS")
    icns_sizes = [16, 32, 64, 128, 256, 512, 1024]
    
    iconset_dir = f'{output_dir}/icon.iconset'
    os.makedirs(iconset_dir, exist_ok=True)
    
    for size in icns_sizes:
        icon = create_app_icon(size)
        icon.save(f'{iconset_dir}/icon_{size}x{size}.png')
        
        if size <= 512:
            icon_2x = create_app_icon(size * 2)
            icon_2x.save(f'{iconset_dir}/icon_{size}x{size}@2x.png')
    
    try:
        import subprocess
        subprocess.run(['iconutil', '-c', 'icns', iconset_dir, '-o', f'{output_dir}/icon.icns'], check=True)
        print("   ✅ 保存: icon.icns")
        
        import shutil
        shutil.rmtree(iconset_dir)
    except (subprocess.CalledProcessError, FileNotFoundError):
        print("   ⚠️  iconutil 不可用（需要 macOS）")
    
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
    
    print("\n🪟 生成 Windows 托盘 ICO")
    tray_ico_sizes = [(16, 16), (32, 32), (48, 48)]
    tray_ico_images = [create_tray_icon(size[0]) for size in tray_ico_sizes]
    tray_ico_images[0].save(f'{output_dir}/icon_tray.ico', format='ICO', sizes=tray_ico_sizes)
    print("   ✅ 保存: icon_tray.ico")
    
    print("\n🏪 生成 Windows Store 图标")
    store_icon = create_app_icon(310)
    store_icon.save(f'{output_dir}/Square310x310Logo.png')
    print("   ✅ 保存: Square310x310Logo.png")
    
    print("\n" + "=" * 50)
    print("✨ 所有图标生成完成！")
    print(f"📁 输出目录: {output_dir}")
    print("\n图标特点:")
    print("  • 流线型弧度 V 字母（贝塞尔曲线）")
    print("  • 更亮的渐变：亮青色→亮蓝紫色→亮粉紫色")
    print("  • 3D 立体阴影和高光")
    print("  • 纯白色背景 (255, 255, 255)")
    print("  • 平滑抗锯齿，高质量渲染")

if __name__ == '__main__':
    main()
