<?php

namespace Illuminate\Validation\Rules;

use Illuminate\Contracts\Validation\DataAwareRule;
use Illuminate\Contracts\Validation\Rule;
use Illuminate\Contracts\Validation\ValidatorAwareRule;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\Validator;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;

class File implements Rule, DataAwareRule, ValidatorAwareRule
{
    use Conditionable, Macroable;

    /**
     * The MIME types that the given file should match. This array may also contain file extensions.
     *
     * @var array
     */
    protected $allowedMimetypes = [];

    /**
     * The extensions that the given file should match.
     *
     * @var array
     */
    protected $allowedExtensions = [];

    /**
     * The minimum size in kilobytes that the file can be.
     *
     * @var null|int|float
     */
    protected $minimumFileSize = null;

    /**
     * The maximum size in kilobytes that the file can be.
     *
     * @var null|int|float
     */
    protected $maximumFileSize = null;

    /**
     * The required file encoding.
     *
     * @var string|null
     */
    protected $encoding = null;

    /**
     * An array of custom rules that will be merged into the validation rules.
     *
     * @var array
     */
    protected $customRules = [];

    /**
     * The error message after validation, if any.
     *
     * @var array
     */
    protected $messages = [];

    /**
     * The data under validation.
     *
     * @var array
     */
    protected $data;

    /**
     * The validator performing the validation.
     *
     * @var \Illuminate\Validation\Validator
     */
    protected $validator;

    /**
     * The callback that will generate the "default" version of the file rule.
     *
     * @var string|array|callable|null
     */
    public static $defaultCallback;

    /**
     * Set the default callback to be used for determining the file default rules.
     *
     * If no arguments are passed, the default file rule configuration will be returned.
     *
     * @param  static|callable|null  $callback
     * @return ($callback is null ? static : void)
     *
     * @throws \InvalidArgumentException
     */
    public static function defaults($callback = null)
    {}

    /**
     * Get the default configuration of the file rule.
     *
     * @return static
     */
    public static function default()
    {}

    /**
     * Limit the uploaded file to only image types.
     *
     * @param  bool  $allowSvg
     * @return ImageFile
     */
    public static function image($allowSvg = false)
    {}

    /**
     * Limit the uploaded file to the given MIME types or file extensions.
     *
     * @param  string|array<int, string>  $mimetypes
     * @return static
     */
    public static function types($mimetypes)
    {}

    /**
     * Limit the uploaded file to the given file extensions.
     *
     * @param  string|array<int, string>  $extensions
     * @return $this
     */
    public function extensions($extensions)
    {}

    /**
     * Indicate that the uploaded file should be exactly a certain size in kilobytes.
     *
     * @param  string|int  $size
     * @return $this
     */
    public function size($size)
    {}

    /**
     * Indicate that the uploaded file should be between a minimum and maximum size in kilobytes.
     *
     * @param  string|int  $minSize
     * @param  string|int  $maxSize
     * @return $this
     */
    public function between($minSize, $maxSize)
    {}

    /**
     * Indicate that the uploaded file should be no less than the given number of kilobytes.
     *
     * @param  string|int  $size
     * @return $this
     */
    public function min($size)
    {}

    /**
     * Indicate that the uploaded file should be no more than the given number of kilobytes.
     *
     * @param  string|int  $size
     * @return $this
     */
    public function max($size)
    {}

    /**
     * Indicate that the uploaded file should be in the given encoding.
     *
     * @param  string  $encoding
     * @return $this
     */
    public function encoding($encoding)
    {}

    /**
     * Convert a potentially human-friendly file size to kilobytes.
     *
     * @param  string|int  $size
     * @return ($size is int ? int : int|float)
     *
     * @throws \InvalidArgumentException
     */
    protected function toKilobytes($size)
    {}

    /**
     * Specify additional validation rules that should be merged with the default rules during validation.
     *
     * @param  string|array  $rules
     * @return $this
     */
    public function rules($rules)
    {}

    /**
     * Determine if the validation rule passes.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function passes($attribute, $value)
    {}

    /**
     * Build the array of underlying validation rules based on the current state.
     *
     * @return array
     */
    protected function buildValidationRules()
    {}

    /**
     * Separate the given MIME types from extensions and return an array of correct rules to validate against.
     *
     * @return array
     */
    protected function buildMimetypes()
    {}

    /**
     * Adds the given failures, and return false.
     *
     * @param  array|string  $messages
     * @return bool
     */
    protected function fail($messages)
    {}

    /**
     * Get the validation error message.
     *
     * @return array
     */
    public function message()
    {}

    /**
     * Set the current validator.
     *
     * @param  \Illuminate\Contracts\Validation\Validator  $validator
     * @return $this
     */
    public function setValidator($validator)
    {}

    /**
     * Set the current data under validation.
     *
     * @param  array  $data
     * @return $this
     */
    public function setData($data)
    {}
}
