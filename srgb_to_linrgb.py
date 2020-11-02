#! /usr/bin/env python
# -*- coding: utf-8 -*-

def sRGB_to_linRGB(s):
    if s <= 0.04045:
        l = s / 12.92
    else:
        l = pow((s + 0.055) / 1.055, 2.4)
    return l


print('Convert: sRGB -> linear RGB')
print('Input the color as "r g b".')
print()
print('Select the input format:')
print('[0] - 0.1 0.1 0.1')
print('[1] - 32 32 32')
print('[2] - EECCFF')
in_format = int(input('input format: '))
srgb = input('sRGB color: ')
if in_format == 0:
    srgb = srgb.split(' ')
    lin_rgb = tuple(sRGB_to_linRGB(float(x)) for x in srgb)
elif in_format == 1:
    srgb = srgb.split(' ')
    lin_rgb = tuple(sRGB_to_linRGB(int(x) / 256.0) for x in srgb)
elif in_format == 2:
    srgb_int = tuple(int(srgb[i*2:(i+1)*2], 16) for i in range(3))
    lin_rgb = tuple(sRGB_to_linRGB(x / 256.0) for x in srgb_int)
print(lin_rgb)
