#! /usr/bin/env python
# -*- coding: utf-8 -*-

def sRGB_to_linRGB(s):
    if s <= 0.04045:
        l = s / 12.92
    else:
        l = pow((s + 0.055) / 1.055, 2.4)
    return l


print('Convert: sRGB -> linear RGB')
print('Input the color as "r g b" with 0 <= r,g,b <= 1.')
srgb = input('sRGB color: ').split(' ')
lin_rgb = tuple(map(lambda x: sRGB_to_linRGB(float(x)), srgb))
print(lin_rgb)
