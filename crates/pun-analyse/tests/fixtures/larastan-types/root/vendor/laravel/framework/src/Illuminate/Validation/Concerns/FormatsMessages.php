<?php

namespace Illuminate\Validation\Concerns;

use Closure;
use Illuminate\Support\Arr;
use Illuminate\Support\Number;
use Illuminate\Support\Str;
use Symfony\Component\HttpFoundation\File\File;
use Symfony\Component\HttpFoundation\File\UploadedFile;

trait FormatsMessages
{
    use ReplacesAttributes;

    /**
     * Get the validation message for an attribute and rule.
     *
     * @param  string  $attribute
     * @param  string  $rule
     * @return string
     */
    protected function getMessage($attribute, $rule)
    {}

    /**
     * Get the proper inline error message for standard and size rules.
     *
     * @param  string  $attribute
     * @param  string  $rule
     * @return string|null
     */
    protected function getInlineMessage($attribute, $rule)
    {}

    /**
     * Get the inline message for a rule if it exists.
     *
     * @param  string  $attribute
     * @param  string  $lowerRule
     * @param  array|null  $source
     * @return string|null
     */
    protected function getFromLocalArray($attribute, $lowerRule, $source = null)
    {}

    /**
     * Get the custom error message from the translator.
     *
     * @param  array|string  $keys
     * @return string
     */
    protected function getCustomMessageFromTranslator($keys)
    {}

    /**
     * Check the given messages for a wildcard key.
     *
     * @param  array  $messages
     * @param  string  $search
     * @param  string  $default
     * @return string
     */
    protected function getWildcardCustomMessages($messages, $search, $default)
    {}

    /**
     * Get the proper error message for an attribute and size rule.
     *
     * @param  string  $attribute
     * @param  string  $rule
     * @return string
     */
    protected function getSizeMessage($attribute, $rule)
    {}

    /**
     * Get the data type of the given attribute.
     *
     * @param  string  $attribute
     * @return string
     */
    protected function getAttributeType($attribute)
    {}

    /**
     * Replace all error message place-holders with actual values.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array  $parameters
     * @return string
     */
    public function makeReplacements($message, $attribute, $rule, $parameters)
    {}

    /**
     * Get the displayable name of the attribute.
     *
     * @param  string  $attribute
     * @return string
     */
    public function getDisplayableAttribute($attribute)
    {}

    /**
     * Get the given attribute from the attribute translations.
     *
     * @param  string  $name
     * @return string|null
     */
    protected function getAttributeFromTranslations($name)
    {}

    /**
     * Get the custom name for an attribute if it exists in the given array.
     *
     * @param  string  $attribute
     * @param  array|null  $source
     * @return string|null
     */
    protected function getAttributeFromLocalArray($attribute, $source = null)
    {}

    /**
     * Replace the :attribute placeholder in the given message.
     *
     * @param  string  $message
     * @param  string  $value
     * @return string
     */
    protected function replaceAttributePlaceholder($message, $value)
    {}

    /**
     * Replace the :index placeholder in the given message.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @return string
     */
    protected function replaceIndexPlaceholder($message, $attribute)
    {}

    /**
     * Replace the :position placeholder in the given message.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @return string
     */
    protected function replacePositionPlaceholder($message, $attribute)
    {}

    /**
     * Replace the :ordinal-position placeholder in the given message.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @return string
     */
    protected function replaceOrdinalPositionPlaceholder($message, $attribute)
    {}

    /**
     * Replace the :index or :position placeholder in the given message.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $placeholder
     * @param  \Closure|null  $modifier
     * @return string
     */
    protected function replaceIndexOrPositionPlaceholder($message, $attribute, $placeholder, ?Closure $modifier = null)
    {}

    /**
     * Get the word for a index or position segment.
     *
     * @param  int  $value
     * @return string
     */
    protected function numberToIndexOrPositionWord(int $value)
    {}

    /**
     * Replace the :input placeholder in the given message.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @return string
     */
    protected function replaceInputPlaceholder($message, $attribute)
    {}

    /**
     * Get the displayable name of the value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return string
     */
    public function getDisplayableValue($attribute, $value)
    {}

    /**
     * Transform an array of attributes to their displayable form.
     *
     * @param  array  $values
     * @return array
     */
    protected function getAttributeList(array $values)
    {}

    /**
     * Call a custom validator message replacer.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array  $parameters
     * @param  \Illuminate\Validation\Validator  $validator
     * @return string|null
     */
    protected function callReplacer($message, $attribute, $rule, $parameters, $validator)
    {}

    /**
     * Call a class based validator message replacer.
     *
     * @param  string  $callback
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array  $parameters
     * @param  \Illuminate\Validation\Validator  $validator
     * @return string
     */
    protected function callClassBasedReplacer($callback, $message, $attribute, $rule, $parameters, $validator)
    {}
}
