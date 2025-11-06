from PIL import Image, ImageDraw
import os

# 创建输出目录
output_dir = "frame_icons"  # 改名以区分
os.makedirs(output_dir, exist_ok=True)

# PNG 尺寸
sizes_png = [
    (32, "32x32.png"),
    (128, "128x128.png"),
    (256, "128x128@2x.png")
]

# ICO 尺寸（Windows）
ico_sizes = [16, 24, 32, 48, 64, 128, 256]

# ICNS 尺寸（macOS）
icns_sizes = [16, 32, 64, 128, 256, 512, 1024]

def create_frame_icon(size):
    """创建抽帧图标"""
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # 胶片条背景（圆角矩形）
    margin = size // 8
    rect = [margin, margin, size - margin, size - margin]
    radius = size // 6
    draw.rounded_rectangle(rect, radius=radius, fill="#4F46E5")  # 深蓝

    # 胶片孔（左侧和右侧）
    hole_size = size // 20
    hole_positions_y = [size//6, size//3, size//2, 2*size//3]
    for y in hole_positions_y:
        # 左侧
        left_hole = [margin//2, y - hole_size//2, margin//2 + hole_size, y + hole_size//2]
        draw.ellipse(left_hole, fill="black")
        # 右侧
        right_hole = [size - margin//2 - hole_size, y - hole_size//2, size - margin//2, y + hole_size//2]
        draw.ellipse(right_hole, fill="black")

    # 抽帧小矩形（网格，代表提取的帧）
    frame_size = (size - 2*margin) // 4  # 约1/4尺寸
    frame_margin = (size - 2*margin - 2*frame_size) // 3
    for row in range(2):  # 2行
        for col in range(3):  # 3列（总6帧）
            x = margin + col * (frame_size + frame_margin)
            y = margin + row * (frame_size + frame_margin)
            frame_rect = [x, y, x + frame_size, y + frame_size]
            draw.rounded_rectangle(frame_rect, radius=frame_size//4, fill="white")

    # 轻微阴影（立体感）
    shadow = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    s_draw = ImageDraw.Draw(shadow)
    s_draw.rounded_rectangle(rect, radius=radius, fill=(0,0,0,30))
    img = Image.alpha_composite(shadow, img)

    return img

# 生成 PNG
for size, filename in sizes_png:
    img = create_frame_icon(size)
    path = os.path.join(output_dir, filename)
    img.save(path, "PNG")
    print(f"Generated: {path}")

# 生成 ICO
ico_images = [create_frame_icon(s) for s in ico_sizes]
ico_path = os.path.join(output_dir, "icon.ico")
ico_images[0].save(ico_path, format='ICO', sizes=[(s, s) for s in ico_sizes], bitmap_format='bmp')
print(f"Generated: {ico_path}")

# 生成 ICNS
icns_images = [create_frame_icon(s) for s in icns_sizes]
icns_path = os.path.join(output_dir, "icon.icns")
icns_images[-1].save(icns_path, format='ICNS', sizes=[(s, s) for s in icns_sizes])
print(f"Generated: {icns_path}")

print("\n抽帧图标生成完成！目录：", output_dir)