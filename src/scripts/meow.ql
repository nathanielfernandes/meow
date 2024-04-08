external bg = #1f1f1f
external radius = 10.0
external pad = 5
external pad_bottom = 20
external image = none
external image_mh = 512
external filter = "best"
external signature = none
external role_color = none

@EnterFunctionNewContext(false)
@SetFilter(filter)

let width, height = @Dimensions()
let fg = @rgba(0, 0, 0, 0.3)

// @DrawRoundedRectangle(0, 0, width, height, radius)
@DrawRectangle(0, 0, width, height)

@SetColor(bg)
@Fill()

with @PaddingEx(pad, pad, pad_bottom, pad) as w, h {
    @DrawRoundedRectangle(0, 0, w, h, radius - pad / 2)

    @SetColor(fg)
    @Fill()

    if image != none {
        with @Padding(pad) as w, h {
            let x, y, w, h = [0, h - image_mh, w, image_mh]

            @DrawRoundedRectangle(x, y, w, h, radius - pad)
            @ClipPreserve()

            @DrawImageCovered(image, x, y, w, h)

            @SetColor(@rgba(0, 0, 0, 0.8))
            @FillPreserve()

            @DrawImageContained(image, x, y, w, h)
        }
    }
}

if signature != none {
    @SetFont("ggsans-bold")
    @SetFontSize(pad_bottom - pad)

    if role_color == none {
        @DrawStringAnchored(signature, pad * 2, height - pad_bottom + 5, 0.0, 0.2)
        @SetColor(#ffffff)
        @Fill()
    } else {
        role_color = if role_color != #000 { role_color } else { #ffffff }

        let tw = @TextWidth(signature)
        let r, g, b = role_color
        @SetColor(@rgba(r, g, b, 0.3))
        @DrawRoundedRectangle(pad * 2, height - pad_bottom + pad/2, tw + pad * 2, pad_bottom - pad, 4)
        @Fill()

        @DrawStringAnchored(signature, pad * 3, height - pad_bottom + 5, 0.0, 0.2)
        @SetColor(role_color)
        @Fill()
    }
}