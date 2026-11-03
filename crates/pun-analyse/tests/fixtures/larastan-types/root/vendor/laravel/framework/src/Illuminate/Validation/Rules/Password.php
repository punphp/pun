<?php

namespace Illuminate\Validation\Rules;

use ArrayIterator;
use Illuminate\Container\Container;
use Illuminate\Contracts\Validation\DataAwareRule;
use Illuminate\Contracts\Validation\ImplicitRule;
use Illuminate\Contracts\Validation\Rule;
use Illuminate\Contracts\Validation\UncompromisedVerifier;
use Illuminate\Contracts\Validation\ValidatorAwareRule;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\Validator;
use Illuminate\Support\Traits\Conditionable;
use InvalidArgumentException;
use IteratorAggregate;
use Traversable;

class Password implements DataAwareRule, ImplicitRule, IteratorAggregate, Rule, ValidatorAwareRule
{
    use Conditionable;

    /**
     * The validator performing the validation.
     *
     * @var \Illuminate\Contracts\Validation\Validator
     */
    protected $validator;

    /**
     * The data under validation.
     *
     * @var array
     */
    protected $data;

    /**
     * The minimum size of the password.
     *
     * @var int
     */
    protected $min = 8;

    /**
     * The maximum size of the password.
     *
     * @var int
     */
    protected $max;

    /**
     * If the password is required.
     *
     * @var bool
     */
    protected $required = false;

    /**
     * If the password should only be validated when present.
     *
     * @var bool
     */
    protected $sometimes = false;

    /**
     * If the password requires at least one uppercase and one lowercase letter.
     *
     * @var bool
     */
    protected $mixedCase = false;

    /**
     * If the password requires at least one letter.
     *
     * @var bool
     */
    protected $letters = false;

    /**
     * If the password requires at least one number.
     *
     * @var bool
     */
    protected $numbers = false;

    /**
     * If the password requires at least one symbol.
     *
     * @var bool
     */
    protected $symbols = false;

    /**
     * If the password should not have been compromised in data leaks.
     *
     * @var bool
     */
    protected $uncompromised = false;

    /**
     * The number of times a password can appear in data leaks before being considered compromised.
     *
     * @var int
     */
    protected $compromisedThreshold = 0;

    /**
     * Additional validation rules that should be merged into the default rules during validation.
     *
     * @var array
     */
    protected $customRules = [];

    /**
     * The failure messages, if any.
     *
     * @var array
     */
    protected $messages = [];

    /**
     * The callback that will generate the "default" version of the password rule.
     *
     * @var string|array|callable|null
     */
    public static $defaultCallback;

    /**
     * Create a new rule instance.
     *
     * @param  int  $min
     */
    public function __construct($min)
    {}

    /**
     * Set the default callback to be used for determining a password's default rules.
     *
     * If no arguments are passed, the default password rule configuration will be returned.
     *
     * @param  static|callable|null  $callback
     * @return ($callback is null ? static : void)
     *
     * @throws \InvalidArgumentException
     */
    public static function defaults($callback = null)
    {}

    /**
     * Get the default configuration of the password rule.
     *
     * @return static
     */
    public static function default()
    {}

    /**
     * Get the default configuration of the password rule and mark the field as required.
     *
     * @return static
     */
    public static function required()
    {}

    /**
     * Get the default configuration of the password rule and mark the field as sometimes being required.
     *
     * @return static
     */
    public static function sometimes()
    {}

    /**
     * Set the performing validator.
     *
     * @param  \Illuminate\Contracts\Validation\Validator  $validator
     * @return $this
     */
    public function setValidator($validator)
    {}

    /**
     * Set the data under validation.
     *
     * @param  array  $data
     * @return $this
     */
    public function setData($data)
    {}

    /**
     * Set the minimum size of the password.
     *
     * @param  int  $size
     * @return static
     */
    public static function min($size)
    {}

    /**
     * Set the maximum size of the password.
     *
     * @param  int  $size
     * @return $this
     */
    public function max($size)
    {}

    /**
     * Ensures the password has not been compromised in data leaks.
     *
     * @param  int  $threshold
     * @return $this
     */
    public function uncompromised($threshold = 0)
    {}

    /**
     * Makes the password require at least one uppercase and one lowercase letter.
     *
     * @return $this
     */
    public function mixedCase()
    {}

    /**
     * Makes the password require at least one letter.
     *
     * @return $this
     */
    public function letters()
    {}

    /**
     * Makes the password require at least one number.
     *
     * @return $this
     */
    public function numbers()
    {}

    /**
     * Makes the password require at least one symbol.
     *
     * @return $this
     */
    public function symbols()
    {}

    /**
     * Specify additional validation rules that should be merged with the default rules during validation.
     *
     * @param  \Closure|string|array  $rules
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
     * Get the validation error message.
     *
     * @return array
     */
    public function message()
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
     * Get information about the current state of the password validation rules.
     *
     * @return array
     */
    public function appliedRules()
    {}

    /**
     * Convert the password rule to a passwordrules HTML attribute string.
     *
     * @return string
     *
     * @see https://developer.apple.com/password-rules/
     */
    public function toPasswordRulesString()
    {}

    /**
     * Get an iterator for the password validation rules.
     *
     * @return \ArrayIterator<int, mixed>
     */
    public function getIterator(): Traversable
    {}
}
