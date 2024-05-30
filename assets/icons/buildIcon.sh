mkdir MyIcon.iconset
convert -background none -resize 16x16 source.svg MyIcon.iconset/icon_16x16.png
convert -background none -resize 32x32 source.svg MyIcon.iconset/icon_16x16@2x.png  # Retina display size for 16x16
convert -background none -resize 32x32 source.svg MyIcon.iconset/icon_32x32.png
convert -background none -resize 64x64 source.svg MyIcon.iconset/icon_32x32@2x.png  # Retina display size for 32x32
convert -background none -resize 128x128 source.svg MyIcon.iconset/icon_128x128.png
convert -background none -resize 256x256 source.svg MyIcon.iconset/icon_128x128@2x.png  # Retina display size for 128x128
convert -background none -resize 256x256 source.svg MyIcon.iconset/icon_256x256.png
convert -background none -resize 512x512 source.svg MyIcon.iconset/icon_256x256@2x.png  # Retina display size for 256x256
convert -background none -resize 512x512 source.svg MyIcon.iconset/icon_512x512.png
convert -background none -resize 1024x1024 source.svg MyIcon.iconset/icon_512x512@2x.png  # Retina display size for 512x512

#iconutil -c icns MyIcon.iconset