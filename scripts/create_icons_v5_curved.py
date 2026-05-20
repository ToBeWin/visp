#!/usr/bin/env python3
"""
Visp Icon Generator V5 - 流线型弧度设计
特点：
- 弧形 V 字母（贝塞尔曲线）
- 流畅的渐变效果
- 3D 立体感
- 高端现代风格
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
    """
    三次贝塞尔曲线
    p0: 起点
    p1: 第一个控制点
    p2: 第二个控制点
    p3: 终点
    """
    points = []
    for i in range(num_points + 1):
        t = i / num_points
        
        # 贝塞尔曲线公式
        x = (1-t)**3 * p0[0] + 3*(1-t)**2*t * p1[0] + 3*(1-t)*t**2 * p2[0] + t**3 * p3[0]
        y = (1-t)**3 * p0[1] + 3*(1-t)**2*t * p1[1] + 3*(1-t)*t**2 * p2[1] + t**3 * p3[1]
        
        points.append((x, y))
    
    return points

def draw_curved_v_letter(size):
    """
    绘制流线型弧度 V 字母
    使用贝塞尔曲线创建平滑的弧形
    """
    # 创建高分辨率画布（用于抗锯齿）
    scale = 4
    canvas_size = size * scale
    canvas = Image.new('RGBA', (canvas_size, canvas_size), (0, 0, 0, 0))
    
    # V 字母的基本参数
    center_x = canvas_size // 2
    top_y = int(canvas_size * 0.28)      # V 的顶部
    bottom_y = int(canvas_size * 0.75)   # V 的底部
    top_width = int(canvas_size * 0.38)  # 顶部宽度
    thickness = int(canvas_size * 0.11)  # 粗细
    
    # === 绘制左边的弧形带 ===
    
    # 左边外侧曲线（从左上到底部中心）
    left_outer_start = (center_x - top_width, top_y)
    left_outer_end = (center_x, bottom_y)
    
    # 控制点（创建向内弯曲的效果）
    left_outer_ctrl1 = (center_x - top_width * 0.8, top_y + (bottom_y - top_y) * 0.3)
    left_outer_ctrl2 = (center_x - top_width * 0.3, top_y + (bottom_y - top_y) * 0.7)
    
    left_outer_curve = bezier_curve(
        left_outer_start, 
        left_outer_ctrl1, 
        left_outer_ctrl2, 
        left_outer_end, 
        num_points=200
    )
    
    # 左边内侧曲线（平行于外侧，但更靠内）
    left_inner_start = (center_x - top_width + thickness, top_y)
    left_inner_end = (center_x + thickness // 2, bottom_y)
    
    left_inner_ctrl1 = (center_x - top_width * 0.8 + thickness, top_y + (bottom_y - top_y) * 0.3)
    left_inner_ctrl2 = (center_x - top_width * 0.3 + thickness, top_y + (bottom_y - top_y) * 0.7)
    
    left_inner_curve = bezier_curve(
        left_inner_start,
        left_inner_ctrl1,
        left_inner_ctrl2,
        left_inner_end,
        num_points=200
    )
    
    # === 绘制右边的弧形带 ===
    
    # 右边外侧曲线
    right_outer_start = (center_x + top_width, top_y)
    right_outer_end = (center_x, bottom_y)
    
    right_outer_ctrl1 = (center_x + top_width * 0.8, top_y + (bottom_y - top_y) * 0.3)
    right_outer_ctrl2 = (center_x + top_width * 0.3, top_y + (bottom_y - top_y) * 0.7)
    
    right_outer_curve = bezier_curve(
        right_outer_start,
        right_outer_ctrl1,
        right_outer_ctrl2,
        right_outer_end,
        num_points=200
    )
    
    # 右边内侧曲线
    right_inner_start = (center_x + top_width - thickness, top_y)
    right_inner_end = (center_x - thickness // 2, bottom_y)
    
    right_inner_ctrl1 = (center_x + top_width * 0.8 - thickness, top_y + (bottom_y - top_y) * 0.3)
    right_inner_ctrl2 = (center_x + top_width * 0.3 - thickness, top_y + (bottom_y - top_y) * 0.7)
    
    right_inner_curve = bezier_curve(
        right_inner_start,
        right_inner_ctrl1,
        right_inner_ctrl2,
        right_inner_end,
        num_points=200
    )
    
    # === 创建渐变填充 ===
    
    # 左边部分：青色到蓝紫色渐变
    left_part = Image.new('RGBA', (canvas_size, canvas_size), (0, 0, 0, 0))
    left_draw = ImageDraw.Draw(left_part)
    
    # 构建左边的多边形（外侧曲线 + 内侧曲线反向）
    left_polygon = left_outer_curve + list(reversed(left_inner_curve))
    
    # 绘制左边的渐变
    for y in range(top_y, bottom_y + 1):
        # 找到这一行的左右边界
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
            # 计算渐变颜色（从青色到蓝紫色）
            ratio = (y - top_y) / (bottom_y - top_y)
            
            # 青色 #4FC3F7 到蓝紫色 #7E57C2
            r = int(79 * (1 - ratio) + 126 * ratio)
            g = int(195 * (1 - ratio) + 87 * ratio)
            b = int(247 * (1 - ratio) + 194 * ratio)
            
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
            # 蓝紫色 #7E57C2 到粉紫色 #EC407A
            ratio = (y - top_y) / (bottom_y - top_y)
            
            r = int(126 * (1 - ratio) + 236 * ratio)
            g = int(87 * (1 - ratio) + 64 * ratio)
            b = int(194 * (1 - ratio) + 122 * ratio)
            
            right_draw.line([(int(inner_x), y), (int(outer_x), y)], fill=(r, g, b, 255), width=1)
    
    # 合并左右部分
    canvas = Image.alpha_composite(canvas, left_part)
    canvas = Image.alpha_composite(canvas, right_part)
    
    # === 添加阴影 ===
    shadow = Image.new('RGBA', (canvas_size, canvas_size), (0, 0, 0, 0))
    shadow_draw = ImageDraw.Draw(shadow)
    
    # 左边阴影
    shadow_offset = int(canvas_size * 0.012)
    left_shadow_polygon = [(x + shadow_offset, y + shadow_offset) for x, y in left_polygon]
    shadow_draw.polygon(left_shadow_polygon, fill=(0, 0, 0, 60))
    
    # 右边阴影
    right_polygon = right_outer_curve + list(reversed(right_inner_curve))
    right_shadow_polygon = [(x + shadow_offset, y + shadow_offset) for x, y in right_polygon]
    shadow_draw.polygon(right_shadow_polygon, fill=(0, 0, 0, 60))
    
    # 模糊阴影
    shadow = shadow.filter(ImageFilter.GaussianBlur(radius=canvas_size * 0.008))
    
    # 将阴影放在最底层
    final = Image.new('RGBA', (canvas_size, canvas_size), (0, 0, 0, 0))
    final = Image.alpha_composite(final, shadow)
    final = Image.alpha_composite(final, canvas)
    
    # === 添加高光 ===
    highlight = Image.new('RGBA', (canvas_size, canvas_size), (0, 0, 0, 0))
    highlight_draw = ImageDraw.Draw(highlight)
    
    # 在顶部添加柔和高光
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
    
    # 缩小到目标尺寸（抗锯齿）
    final = final.resize((size, size), Image.Resampling.LANCZOS)
    
    return final

def create_app_icon(size):
    """创建应用图标"""
    # 优化后的背景色：更深的灰蓝色，更高端
    bg_color = (28, 32, 38, 255)  # 深灰蓝色
    
    # 创建圆角矩形背景
    radius = int(size * 0.225)
    background = create_rounded_square(size, radius, bg_color)
    
    # 绘制流线型 V 字母
    v_letter = draw_curved_v_letter(size)
    
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
    
    print("🎨 生成 Visp 图标 V5 - 流线型弧度设计")
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
    print("  • 流线型弧度 V 字母（贝塞尔曲线）")
    print("  • 青色→蓝紫色→粉紫色渐变")
    print("  • 3D 立体阴影和高光")
    print("  • 深灰蓝色背景，高端现代")
    print("  • 平滑抗锯齿，高质量渲染")

if __name__ == '__main__':
    main()
