<?php

namespace Illuminate\Validation\Rules;

class ImageFile extends File
{
    /**
     * Create a new image file rule instance.
     *
     * @param  bool  $allowSvg
     */
    public function __construct($allowSvg = false)
    {}

    /**
     * The dimension constraints for the uploaded file.
     *
     * @param  \Illuminate\Validation\Rules\Dimensions  $dimensions
     * @return $this
     */
    public function dimensions($dimensions)
    {}
}
