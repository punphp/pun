<?php

namespace Illuminate\Validation\Rules;

use Illuminate\Support\Traits\Conditionable;
use Stringable;

class Dimensions implements Stringable
{
    use Conditionable;

    /**
     * The constraints for the dimensions rule.
     *
     * @var array
     */
    protected $constraints = [];

    /**
     * Create a new dimensions rule instance.
     *
     * @param  array  $constraints
     */
    public function __construct(array $constraints = [])
    {}

    /**
     * Set the "width" constraint.
     *
     * @param  int  $value
     * @return $this
     */
    public function width($value)
    {}

    /**
     * Set the "height" constraint.
     *
     * @param  int  $value
     * @return $this
     */
    public function height($value)
    {}

    /**
     * Set the "min width" constraint.
     *
     * @param  int  $value
     * @return $this
     */
    public function minWidth($value)
    {}

    /**
     * Set the "min height" constraint.
     *
     * @param  int  $value
     * @return $this
     */
    public function minHeight($value)
    {}

    /**
     * Set the "max width" constraint.
     *
     * @param  int  $value
     * @return $this
     */
    public function maxWidth($value)
    {}

    /**
     * Set the "max height" constraint.
     *
     * @param  int  $value
     * @return $this
     */
    public function maxHeight($value)
    {}

    /**
     * Set the "ratio" constraint.
     *
     * @param  float  $value
     * @return $this
     */
    public function ratio($value)
    {}

    /**
     * Set the minimum aspect ratio.
     *
     * @param  float  $value
     * @return $this
     */
    public function minRatio($value)
    {}

    /**
     * Set the maximum aspect ratio.
     *
     * @param  float  $value
     * @return $this
     */
    public function maxRatio($value)
    {}

    /**
     * Set the aspect ratio range.
     *
     * @param  float  $min
     * @param  float  $max
     * @return $this
     */
    public function ratioBetween($min, $max)
    {}

    /**
     * Convert the rule to a validation string.
     *
     * @return string
     */
    public function __toString()
    {}
}
