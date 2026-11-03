<?php

namespace Illuminate\Validation\Concerns;

use Illuminate\Support\Arr;
use Illuminate\Support\Str;

trait ReplacesAttributes
{
    /**
     * Replace all place-holders for the accepted_if rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceAcceptedIf($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the declined_if rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceDeclinedIf($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the between rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceBetween($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the date_format rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceDateFormat($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the decimal rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,int>  $parameters
     * @return string
     */
    protected function replaceDecimal($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the different rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceDifferent($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the digits rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceDigits($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the digits (between) rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceDigitsBetween($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the encoding rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceEncoding($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the extensions rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceExtensions($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the min rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceMin($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the min digits rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceMinDigits($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the max rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceMax($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the max digits rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceMaxDigits($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the missing_if rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceMissingIf($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the missing_unless rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceMissingUnless($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the missing_with rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceMissingWith($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the missing_with_all rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceMissingWithAll($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the multiple_of rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceMultipleOf($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the in rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceIn($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the not_in rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceNotIn($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the in_array rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceInArray($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the in_array_keys rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceInArrayKeys($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the array_keys rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceArrayKeys($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the required_array_keys rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceRequiredArrayKeys($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the mimetypes rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceMimetypes($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the mimes rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceMimes($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the present_if rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replacePresentIf($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the present_unless rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replacePresentUnless($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the present_with rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replacePresentWith($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the present_with_all rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replacePresentWithAll($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the required_with rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceRequiredWith($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the required_with_all rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceRequiredWithAll($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the required_without rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceRequiredWithout($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the required_without_all rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceRequiredWithoutAll($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the size rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceSize($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the gt rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceGt($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the lt rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceLt($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the gte rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceGte($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the lte rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceLte($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the required_if rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceRequiredIf($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the required_if_accepted rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceRequiredIfAccepted($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the required_if_declined rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceRequiredIfDeclined($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the required_unless rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceRequiredUnless($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the prohibited_if rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceProhibitedIf($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the prohibited_if_accepted rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceProhibitedIfAccepted($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the prohibited_if_declined rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceProhibitedIfDeclined($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the prohibited_unless rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceProhibitedUnless($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the prohibited_with rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceProhibits($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the same rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceSame($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the before rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceBefore($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the before_or_equal rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceBeforeOrEqual($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the after rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceAfter($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the after_or_equal rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceAfterOrEqual($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the date_equals rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceDateEquals($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the dimensions rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceDimensions($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the ends_with rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceEndsWith($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the doesnt_end_with rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceDoesntEndWith($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the starts_with rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceStartsWith($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the doesnt_start_with rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceDoesntStartWith($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace all place-holders for the doesnt_contain rule.
     *
     * @param  string  $message
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array<int,string>  $parameters
     * @return string
     */
    protected function replaceDoesntContain($message, $attribute, $rule, $parameters)
    {}

    /**
     * Replace the given string while maintaining different casing variants.
     *
     * @param  array<string, string>  $mapping
     */
    private function replaceWhileKeepingCase(string $message, array $mapping): string
    {}
}
