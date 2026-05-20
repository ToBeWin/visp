#!/usr/bin/env python3
"""
为 macOS 菜单栏创建专用图标
特点：
- 单色（黑色或白色）
- 透明背景
- 简洁的 V 字母轮廓
- 适配浅色和深色模式
"""

from PIL import Image, ImageDraw

def create_menubar_icon(size):
    """
    创建 macOS 菜单栏图标
    使用 Template Image 格式（黑色 + alpha 通道）
    """
    # 创建透明背景
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # V 字母参数
    center_x = size // 2
    top_y = int(size * 0.20)
    bottom_y = int(size * 0.80)
    width = int(size * 0.35)
    thickness = max(int(size * 0.18), 2)
    
    # 左右端点
    left_top = (center_x - width, top_y)
    right_top = (center_x + width, top_y)
    bottom = (center_x, bottom_y)
    
    # 使用黑色绘制（macOS 会自动处理颜色反转）
    color = (0, 0, 0, 255)
    
    # 绘制左边
    draw.line([left_top, bottom], fill=color, width=thickness)
    
    # 绘制右边
    draw.line([right_top, bottom], fill=color, width=thickness)
    
    return img

def main():
    import os
    
    output_dir = 'src-tauri/icons'
    os.makedirs(output_dir, exist_ok=True)
    
    print("🔔 生成 macOS 菜单栏图标")
    print("=" * 50)
    
    # macOS 菜单栏需要的尺寸
    # Template Image 格式：黑色 + alpha 通道
    sizes = [16, 18, 22, 32, 44, 64]
    
    for size in sizes:
        print(f"   生成 {size}x{size}")
        icon = create_menubar_icon(size)
        
        # 标准尺寸
        if size == 22:
            icon.save(f'{output_dir}/icon_tray.png')
            print(f"      ✅ icon_tray.png (主菜单栏图标)")
        
        icon.save(f'{output_dir}/icon_tray_{size}x{size}.png')
        
        # Retina 版本（标记为 @2x）
        if size in [16, 18, 22, 32]:
            icon.save(f'{output_dir}/icon_tray_{size}x{size}@2x.png')
    
    print("\n" + "=" * 50)
    print("✨ 菜单栏图标生成完成！")
    print(f"📁 输出目录: {output_dir}")
    print("\n特点:")
    print("  • 单色黑色 V 字母")
    print("  • 透明背景")
    print("  • Template Image 格式")
    print("  • 自动适配浅色/深色模式")
    print("  • 简洁清晰")

if __name__ == '__main__':
    main()
