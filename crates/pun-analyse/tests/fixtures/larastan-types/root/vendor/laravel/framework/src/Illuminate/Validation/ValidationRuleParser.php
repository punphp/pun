<?php

namespace Illuminate\Validation;

use Closure;
use Illuminate\Contracts\Validation\CompilableRules;
use Illuminate\Contracts\Validation\InvokableRule;
use Illuminate\Contracts\Validation\ParameterizedRule;
use Illuminate\Contracts\Validation\Rule as RuleContract;
use Illuminate\Contracts\Validation\ValidationRule;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Validation\Rules\ArrayRule;
use Illuminate\Validation\Rules\Date;
use Illuminate\Validation\Rules\Exists;
use Illuminate\Validation\Rules\Numeric;
use Illuminate\Validation\Rules\StringRule;
use Illuminate\Validation\Rules\Unique;

class ValidationRuleParser
{
    /**
     * The data being validated.
     *
     * @var array
     */
    public $data;

    /**
     * The implicit attributes.
     *
     * @var array
     */
    public $implicitAttributes = [];

    /**
     * Create a new validation rule parser.
     *
     * @param  array  $data
     */
    public function __construct(array $data)
    {}

    /**
     * Parse the human-friendly rules into a full rules array for the validator.
     *
     * @param  array  $rules
     * @return \stdClass
     */
    public function explode($rules)
    {}

    /**
     * Explode the rules into an array of explicit rules.
     *
     * @param  array  $rules
     * @return array
     */
    protected function explodeRules($rules)
    {}

    /**
     * Explode the explicit rule into an array if necessary.
     *
     * @param  mixed  $rule
     * @param  string  $attribute
     * @return array
     */
    protected function explodeExplicitRule($rule, $attribute)
    {}

    /**
     * Prepare the given rule for the Validator.
     *
     * @param  mixed  $rule
     * @param  string  $attribute
     * @return mixed
     */
    protected function prepareRule($rule, $attribute)
    {}

    /**
     * Define a set of rules that apply to each element in an array attribute.
     *
     * @param  array  $results
     * @param  string  $attribute
     * @param  string|array  $rules
     * @return array
     */
    protected function explodeWildcardRules($results, $attribute, $rules)
    {}

    /**
     * Merge additional rules into a given attribute(s).
     *
     * @param  array  $results
     * @param  string|array  $attribute
     * @param  string|array  $rules
     * @return array
     */
    public function mergeRules($results, $attribute, $rules = [])
    {}

    /**
     * Merge additional rules into a given attribute.
     *
     * @param  array  $results
     * @param  string  $attribute
     * @param  string|array  $rules
     * @return array
     */
    protected function mergeRulesForAttribute($results, $attribute, $rules)
    {}

    /**
     * Merge additional rules into a given attribute by reference.
     *
     * @param  array  $results
     * @param  string  $attribute
     * @param  string|array  $rules
     * @return void
     */
    private function mergeRulesForAttributeInto(&$results, $attribute, $rules)
    {}

    /**
     * Extract the rule name and parameters from a rule.
     *
     * @param  array|string  $rule
     * @return array
     */
    public static function parse($rule)
    {}

    /**
     * Parse an array based rule.
     *
     * @param  array  $rule
     * @return array
     */
    protected static function parseArrayRule(array $rule)
    {}

    /**
     * Parse a string based rule.
     *
     * @param  string  $rule
     * @return array
     */
    protected static function parseStringRule($rule)
    {}

    /**
     * Parse a parameter list.
     *
     * @param  string  $rule
     * @param  string  $parameter
     * @return array
     */
    protected static function parseParameters($rule, $parameter)
    {}

    /**
     * Determine if the rule is a regular expression.
     *
     * @param  string  $rule
     * @return bool
     */
    protected static function ruleIsRegex($rule)
    {}

    /**
     * Normalizes a rule so that we can accept short types.
     *
     * @param  string  $rule
     * @return string
     */
    protected static function normalizeRule($rule)
    {}

    /**
     * Expand the conditional rules in the given array of rules.
     *
     * @param  array  $rules
     * @param  array  $data
     * @return array
     */
    public static function filterConditionalRules($rules, array $data = [])
    {}
}
