<?php

namespace Illuminate\Validation;

use BadMethodCallException;
use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Translation\Translator;
use Illuminate\Contracts\Validation\DataAwareRule;
use Illuminate\Contracts\Validation\ImplicitRule;
use Illuminate\Contracts\Validation\Rule as RuleContract;
use Illuminate\Contracts\Validation\Validator as ValidatorContract;
use Illuminate\Contracts\Validation\ValidatorAwareRule;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Fluent;
use Illuminate\Support\MessageBag;
use Illuminate\Support\Str;
use Illuminate\Support\ValidatedInput;
use InvalidArgumentException;
use RuntimeException;
use stdClass;
use Symfony\Component\HttpFoundation\File\UploadedFile;

class Validator implements ValidatorContract
{
    use Concerns\FormatsMessages,
        Concerns\ValidatesAttributes;

    /**
     * The Translator implementation.
     *
     * @var \Illuminate\Contracts\Translation\Translator
     */
    protected $translator;

    /**
     * The container instance.
     *
     * @var \Illuminate\Contracts\Container\Container
     */
    protected $container;

    /**
     * The Presence Verifier implementation.
     *
     * @var \Illuminate\Validation\PresenceVerifierInterface|null
     */
    protected $presenceVerifier;

    /**
     * The failed validation rules.
     *
     * @var array
     */
    protected $failedRules = [];

    /**
     * Attributes that should be excluded from the validated data.
     *
     * @var array<string, string>
     */
    protected $excludeAttributes = [];

    /**
     * The message bag instance.
     *
     * @var \Illuminate\Support\MessageBag
     */
    protected $messages;

    /**
     * The data under validation.
     *
     * @var array
     */
    protected $data;

    /**
     * The initial rules provided.
     *
     * @var array
     */
    protected $initialRules;

    /**
     * The rules to be applied to the data.
     *
     * @var array
     */
    protected $rules;

    /**
     * The current rule that is validating.
     *
     * @var string
     */
    protected $currentRule;

    /**
     * The array of wildcard attributes with their asterisks expanded.
     *
     * @var array
     */
    protected $implicitAttributes = [];

    /**
     * The callback that should be used to format the attribute.
     *
     * @var callable|null
     */
    protected $implicitAttributesFormatter;

    /**
     * The cached data for the "distinct" rule.
     *
     * @var array
     */
    protected $distinctValues = [];

    /**
     * All of the registered "after" callbacks.
     *
     * @var array
     */
    protected $after = [];

    /**
     * The array of custom error messages.
     *
     * @var array
     */
    public $customMessages = [];

    /**
     * The array of fallback error messages.
     *
     * @var array
     */
    public $fallbackMessages = [];

    /**
     * The array of custom attribute names.
     *
     * @var array
     */
    public $customAttributes = [];

    /**
     * The array of custom displayable values.
     *
     * @var array
     */
    public $customValues = [];

    /**
     * Indicates if the validator should stop on the first rule failure.
     *
     * @var bool
     */
    protected $stopOnFirstFailure = false;

    /**
     * Indicates that unvalidated array keys should be excluded, even if the parent array was validated.
     *
     * @var bool
     */
    public $excludeUnvalidatedArrayKeys = false;

    /**
     * All of the custom validator extensions.
     *
     * @var array
     */
    public $extensions = [];

    /**
     * All of the custom replacer extensions.
     *
     * @var array
     */
    public $replacers = [];

    /**
     * The validation rules that may be applied to files.
     *
     * @var string[]
     */
    protected $fileRules = [
        'Between',
        'Dimensions',
        'Encoding',
        'Extensions',
        'File',
        'Image',
        'Max',
        'Mimes',
        'Mimetypes',
        'Min',
        'Size',
    ];

    /**
     * The validation rules that imply the field is required.
     *
     * @var string[]
     */
    protected $implicitRules = [
        'Accepted',
        'AcceptedIf',
        'Declined',
        'DeclinedIf',
        'Filled',
        'Missing',
        'MissingIf',
        'MissingUnless',
        'MissingWith',
        'MissingWithAll',
        'Present',
        'PresentIf',
        'PresentUnless',
        'PresentWith',
        'PresentWithAll',
        'Required',
        'RequiredIf',
        'RequiredIfAccepted',
        'RequiredIfDeclined',
        'RequiredUnless',
        'RequiredWith',
        'RequiredWithAll',
        'RequiredWithout',
        'RequiredWithoutAll',
    ];

    /**
     * The validation rules which depend on other fields as parameters.
     *
     * @var string[]
     */
    protected $dependentRules = [
        'After',
        'AfterOrEqual',
        'Before',
        'BeforeOrEqual',
        'Confirmed',
        'Different',
        'ExcludeIf',
        'ExcludeUnless',
        'ExcludeWith',
        'ExcludeWithout',
        'Gt',
        'Gte',
        'Lt',
        'Lte',
        'AcceptedIf',
        'DeclinedIf',
        'RequiredIf',
        'RequiredIfAccepted',
        'RequiredIfDeclined',
        'RequiredUnless',
        'RequiredWith',
        'RequiredWithAll',
        'RequiredWithout',
        'RequiredWithoutAll',
        'PresentIf',
        'PresentUnless',
        'PresentWith',
        'PresentWithAll',
        'Prohibited',
        'ProhibitedIf',
        'ProhibitedIfAccepted',
        'ProhibitedIfDeclined',
        'ProhibitedUnless',
        'Prohibits',
        'MissingIf',
        'MissingUnless',
        'MissingWith',
        'MissingWithAll',
        'Same',
        'Unique',
    ];

    /**
     * The validation rules that can exclude an attribute.
     *
     * @var string[]
     */
    protected $excludeRules = ['Exclude', 'ExcludeIf', 'ExcludeUnless', 'ExcludeWith', 'ExcludeWithout'];

    /**
     * The size related validation rules.
     *
     * @var string[]
     */
    protected $sizeRules = ['Size', 'Between', 'Min', 'Max', 'Gt', 'Lt', 'Gte', 'Lte'];

    /**
     * The numeric related validation rules.
     *
     * @var string[]
     */
    protected $numericRules = ['Numeric', 'Integer', 'Decimal'];

    /**
     * The default numeric related validation rules.
     *
     * @var string[]
     */
    protected $defaultNumericRules = ['Numeric', 'Integer', 'Decimal'];

    /**
     * The current random hash for the validator.
     *
     * @var string|null
     */
    protected static $placeholderHash;

    /**
     * Indicates if DNS lookups performed by validation rules should be faked to always succeed.
     *
     * @var bool
     */
    protected static $fakeDnsLookups = false;

    /**
     * The exception to throw upon failure.
     *
     * @var class-string<\Illuminate\Validation\ValidationException>
     */
    protected $exception = ValidationException::class;

    /**
     * The custom callback to determine if an exponent is within allowed range.
     *
     * @var callable|null
     */
    protected $ensureExponentWithinAllowedRangeUsing;

    /**
     * Create a new Validator instance.
     *
     * @param  \Illuminate\Contracts\Translation\Translator  $translator
     * @param  array  $data
     * @param  array  $rules
     * @param  array  $messages
     * @param  array  $attributes
     */
    public function __construct(
        Translator $translator,
        array $data,
        array $rules,
        array $messages = [],
        array $attributes = [],
    ) {}

    /**
     * Parse the data array, converting dots and asterisks.
     *
     * @param  array  $data
     * @return array
     */
    public function parseData(array $data)
    {}

    /**
     * Replace the placeholders used in data keys.
     *
     * @param  array  $data
     * @return array
     */
    protected function replacePlaceholders($data)
    {}

    /**
     * Replace the placeholders in the given string.
     *
     * @param  string  $value
     * @return string
     */
    protected function replacePlaceholderInString(string $value)
    {}

    /**
     * Replace each field parameter dot placeholder with dot.
     *
     * @param  array  $parameters
     * @return array
     */
    protected function replaceDotPlaceholderInParameters(array $parameters)
    {}

    /**
     * Add an after validation callback.
     *
     * @param  callable|array|string  $callback
     * @return $this
     */
    public function after($callback)
    {}

    /**
     * Determine if the data passes the validation rules.
     *
     * @return bool
     */
    public function passes()
    {}

    /**
     * Determine if the data fails the validation rules.
     *
     * @return bool
     */
    public function fails()
    {}

    /**
     * Execute the callback if the data passes the validation rules.
     *
     * @template TWhenReturnType
     *
     * @param  (callable($this): TWhenReturnType)  $callback
     * @param  (callable($this): TWhenReturnType)|null  $default
     * @return $this|TWhenReturnType
     */
    public function whenPasses(callable $callback, ?callable $default = null)
    {}

    /**
     * Execute the callback if the data fails the validation rules.
     *
     * @template TWhenReturnType
     *
     * @param  (callable($this): TWhenReturnType)  $callback
     * @param  (callable($this): TWhenReturnType)|null  $default
     * @return $this|TWhenReturnType
     */
    public function whenFails(callable $callback, ?callable $default = null)
    {}

    /**
     * Determine if the attribute should be excluded.
     *
     * @param  string  $attribute
     * @return bool
     */
    protected function shouldBeExcluded($attribute)
    {}

    /**
     * Remove the given attribute.
     *
     * @param  string  $attribute
     * @return void
     */
    protected function removeAttribute($attribute)
    {}

    /**
     * Run the validator's rules against its data.
     *
     * @return array
     *
     * @throws \Illuminate\Validation\ValidationException
     */
    public function validate()
    {}

    /**
     * Run the validator's rules against its data.
     *
     * @param  string  $errorBag
     * @return array
     *
     * @throws \Illuminate\Validation\ValidationException
     */
    public function validateWithBag(string $errorBag)
    {}

    /**
     * Get a validated input container for the validated input.
     *
     * @param  array<int, string>|null  $keys
     * @return ($keys is array ? array<string, mixed> : \Illuminate\Support\ValidatedInput)
     *
     * @throws \Illuminate\Validation\ValidationException
     */
    public function safe(?array $keys = null)
    {}

    /**
     * Get the attributes and values that were validated.
     *
     * @return array
     *
     * @throws \Illuminate\Validation\ValidationException
     */
    public function validated()
    {}

    /**
     * Get the rule keys that have nested rules beneath them.
     *
     * @return array<string, true>
     */
    protected function parentRuleKeys()
    {}

    /**
     * Validate a given attribute against a rule.
     *
     * @param  string  $attribute
     * @param  string  $rule
     * @return void
     */
    protected function validateAttribute($attribute, $rule)
    {}

    /**
     * Determine if the given rule depends on other fields.
     *
     * @param  string  $rule
     * @return bool
     */
    protected function dependsOnOtherFields($rule)
    {}

    /**
     * Get the explicit keys from an attribute flattened with dot notation.
     *
     * E.g. 'foo.1.bar.spark.baz' -> [1, 'spark'] for 'foo.*.bar.*.baz'
     *
     * @param  string  $attribute
     * @return array
     */
    protected function getExplicitKeys($attribute)
    {}

    /**
     * Get the primary attribute name.
     *
     * For example, if "name.0" is given, "name.*" will be returned.
     *
     * @param  string  $attribute
     * @return string
     */
    protected function getPrimaryAttribute($attribute)
    {}

    /**
     * Replace each field parameter which has an escaped dot with the dot placeholder.
     *
     * @param  array  $parameters
     * @return array
     */
    protected function replaceDotInParameters(array $parameters)
    {}

    /**
     * Replace each field parameter which has asterisks with the given keys.
     *
     * @param  array  $parameters
     * @param  array  $keys
     * @return array
     */
    protected function replaceAsterisksInParameters(array $parameters, array $keys)
    {}

    /**
     * Determine if the attribute is validatable.
     *
     * @param  object|string  $rule
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    protected function isValidatable($rule, $attribute, $value)
    {}

    /**
     * Determine if the field is present, or the rule implies required.
     *
     * @param  object|string  $rule
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    protected function presentOrRuleIsImplicit($rule, $attribute, $value)
    {}

    /**
     * Determine if a given rule implies the attribute is required.
     *
     * @param  object|string  $rule
     * @return bool
     */
    protected function isImplicit($rule)
    {}

    /**
     * Determine if the attribute passes any optional check.
     *
     * @param  string  $attribute
     * @return bool
     */
    protected function passesOptionalCheck($attribute)
    {}

    /**
     * Determine if the attribute fails the nullable check.
     *
     * @param  string  $rule
     * @param  string  $attribute
     * @return bool
     */
    protected function isNotNullIfMarkedAsNullable($rule, $attribute)
    {}

    /**
     * Determine if it's a necessary presence validation.
     *
     * This is to avoid possible database type comparison errors.
     *
     * @param  string  $rule
     * @param  string  $attribute
     * @return bool
     */
    protected function hasNotFailedPreviousRuleIfPresenceRule($rule, $attribute)
    {}

    /**
     * Validate an attribute using a custom rule object.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  \Illuminate\Contracts\Validation\Rule  $rule
     * @return void
     */
    protected function validateUsingCustomRule($attribute, $value, $rule)
    {}

    /**
     * Check if we should stop further validations on a given attribute.
     *
     * @param  string  $attribute
     * @return bool
     */
    protected function shouldStopValidating($attribute)
    {}

    /**
     * Add a failed rule and error message to the collection.
     *
     * @param  string  $attribute
     * @param  string  $rule
     * @param  array  $parameters
     * @return void
     */
    public function addFailure($attribute, $rule, $parameters = [])
    {}

    /**
     * Add the given attribute to the list of excluded attributes.
     *
     * @param  string  $attribute
     * @return void
     */
    protected function excludeAttribute(string $attribute)
    {}

    /**
     * Returns the data which was valid.
     *
     * @return array
     */
    public function valid()
    {}

    /**
     * Returns the data which was invalid.
     *
     * @return array
     */
    public function invalid()
    {}

    /**
     * Generate an array of all attributes that have messages.
     *
     * @return array
     */
    protected function attributesThatHaveMessages()
    {}

    /**
     * Get the failed validation rules.
     *
     * @return array
     */
    public function failed()
    {}

    /**
     * Get the message container for the validator.
     *
     * @return \Illuminate\Support\MessageBag
     */
    public function messages()
    {}

    /**
     * An alternative more semantic shortcut to the message container.
     *
     * @return \Illuminate\Support\MessageBag
     */
    public function errors()
    {}

    /**
     * Get the messages for the instance.
     *
     * @return \Illuminate\Support\MessageBag
     */
    public function getMessageBag()
    {}

    /**
     * Determine if the given attribute has a rule in the given set.
     *
     * @param  string  $attribute
     * @param  string|array  $rules
     * @return bool
     */
    public function hasRule($attribute, $rules)
    {}

    /**
     * Get a rule and its parameters for a given attribute.
     *
     * @param  string  $attribute
     * @param  string|array  $rules
     * @return array|null
     */
    protected function getRule($attribute, $rules)
    {}

    /**
     * Get the data under validation.
     *
     * @return array
     */
    public function attributes()
    {}

    /**
     * Get the data under validation.
     *
     * @return array
     */
    public function getData()
    {}

    /**
     * Set the data under validation.
     *
     * @param  array  $data
     * @return $this
     */
    public function setData(array $data)
    {}

    /**
     * Get the value of a given attribute.
     *
     * @param  string  $attribute
     * @return mixed
     */
    public function getValue($attribute)
    {}

    /**
     * Set the value of a given attribute.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return void
     */
    public function setValue($attribute, $value)
    {}

    /**
     * Get the validation rules.
     *
     * @return array
     */
    public function getRules()
    {}

    /**
     * Get the validation rules with key placeholders removed.
     *
     * @return array
     */
    public function getRulesWithoutPlaceholders()
    {}

    /**
     * Set the validation rules.
     *
     * @param  array  $rules
     * @return $this
     */
    public function setRules(array $rules)
    {}

    /**
     * Append new validation rules to the validator.
     *
     * @param  array  $rules
     * @return $this
     */
    public function appendRules(array $rules)
    {}

    /**
     * Parse the given rules and merge them into current rules.
     *
     * @internal
     *
     * @param  array  $rules
     * @return void
     */
    public function addRules($rules)
    {}

    /**
     * Add conditions to a given field based on a Closure.
     *
     * @param  string|array  $attribute
     * @param  string|array  $rules
     * @param  callable  $callback
     * @return $this
     */
    public function sometimes($attribute, $rules, callable $callback)
    {}

    /**
     * Get the data that should be injected into the iteration of a wildcard "sometimes" callback.
     *
     * @param  string  $attribute
     * @param  bool  $removeLastSegmentOfAttribute
     * @return \Illuminate\Support\Fluent|mixed
     */
    private function dataForSometimesIteration(string $attribute, bool $removeLastSegmentOfAttribute)
    {}

    /**
     * Instruct the validator to stop validating after the first rule failure.
     *
     * @param  bool  $stopOnFirstFailure
     * @return $this
     */
    public function stopOnFirstFailure($stopOnFirstFailure = true)
    {}

    /**
     * Register an array of custom validator extensions.
     *
     * @param  array  $extensions
     * @return void
     */
    public function addExtensions(array $extensions)
    {}

    /**
     * Register an array of custom implicit validator extensions.
     *
     * @param  array  $extensions
     * @return void
     */
    public function addImplicitExtensions(array $extensions)
    {}

    /**
     * Register an array of custom dependent validator extensions.
     *
     * @param  array  $extensions
     * @return void
     */
    public function addDependentExtensions(array $extensions)
    {}

    /**
     * Register a custom validator extension.
     *
     * @param  string  $rule
     * @param  \Closure|string  $extension
     * @return void
     */
    public function addExtension($rule, $extension)
    {}

    /**
     * Register a custom implicit validator extension.
     *
     * @param  string  $rule
     * @param  \Closure|string  $extension
     * @return void
     */
    public function addImplicitExtension($rule, $extension)
    {}

    /**
     * Register a custom dependent validator extension.
     *
     * @param  string  $rule
     * @param  \Closure|string  $extension
     * @return void
     */
    public function addDependentExtension($rule, $extension)
    {}

    /**
     * Register an array of custom validator message replacers.
     *
     * @param  array  $replacers
     * @return void
     */
    public function addReplacers(array $replacers)
    {}

    /**
     * Register a custom validator message replacer.
     *
     * @param  string  $rule
     * @param  \Closure|string  $replacer
     * @return void
     */
    public function addReplacer($rule, $replacer)
    {}

    /**
     * Set the custom messages for the validator.
     *
     * @param  array  $messages
     * @return $this
     */
    public function setCustomMessages(array $messages)
    {}

    /**
     * Set the custom attributes on the validator.
     *
     * @param  array  $attributes
     * @return $this
     */
    public function setAttributeNames(array $attributes)
    {}

    /**
     * Add custom attributes to the validator.
     *
     * @param  array  $attributes
     * @return $this
     */
    public function addCustomAttributes(array $attributes)
    {}

    /**
     * Set the callback that used to format an implicit attribute.
     *
     * @param  callable|null  $formatter
     * @return $this
     */
    public function setImplicitAttributesFormatter(?callable $formatter = null)
    {}

    /**
     * Set the custom values on the validator.
     *
     * @param  array  $values
     * @return $this
     */
    public function setValueNames(array $values)
    {}

    /**
     * Add the custom values for the validator.
     *
     * @param  array  $customValues
     * @return $this
     */
    public function addCustomValues(array $customValues)
    {}

    /**
     * Set the fallback messages for the validator.
     *
     * @param  array  $messages
     * @return void
     */
    public function setFallbackMessages(array $messages)
    {}

    /**
     * Get the Presence Verifier implementation.
     *
     * @param  string|null  $connection
     * @return \Illuminate\Validation\PresenceVerifierInterface
     *
     * @throws \RuntimeException
     */
    public function getPresenceVerifier($connection = null)
    {}

    /**
     * Set the Presence Verifier implementation.
     *
     * @param  \Illuminate\Validation\PresenceVerifierInterface  $presenceVerifier
     * @return void
     */
    public function setPresenceVerifier(PresenceVerifierInterface $presenceVerifier)
    {}

    /**
     * Get the exception to throw upon failed validation.
     *
     * @return class-string<\Illuminate\Validation\ValidationException>
     */
    public function getException()
    {}

    /**
     * Set the exception to throw upon failed validation.
     *
     * @param  class-string<\Illuminate\Validation\ValidationException>  $exception
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function setException($exception)
    {}

    /**
     * Ensure exponents are within range using the given callback.
     *
     * @param  callable(int, string, mixed): mixed  $callback
     * @return $this
     */
    public function ensureExponentWithinAllowedRangeUsing($callback)
    {}

    /**
     * Get the Translator implementation.
     *
     * @return \Illuminate\Contracts\Translation\Translator
     */
    public function getTranslator()
    {}

    /**
     * Set the Translator implementation.
     *
     * @param  \Illuminate\Contracts\Translation\Translator  $translator
     * @return void
     */
    public function setTranslator(Translator $translator)
    {}

    /**
     * Set the IoC container instance.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return void
     */
    public function setContainer(Container $container)
    {}

    /**
     * Call a custom validator extension.
     *
     * @param  string  $rule
     * @param  array  $parameters
     * @return bool|null
     */
    protected function callExtension($rule, $parameters)
    {}

    /**
     * Call a class based validator extension.
     *
     * @param  string  $callback
     * @param  array  $parameters
     * @return bool
     */
    protected function callClassBasedExtension($callback, $parameters)
    {}

    /**
     * Encode the attribute with the placeholder hash.
     *
     * @param  string  $attribute
     * @return string
     */
    protected static function encodeAttributeWithPlaceholder(string $attribute)
    {}

    /**
     * Decode an attribute with a placeholder hash.
     *
     * @param  string  $attribute
     * @return string
     */
    protected static function decodeAttributeWithPlaceholder(string $attribute)
    {}

    /**
     * Fake the DNS lookups performed by validation rules so they always succeed.
     *
     * @param  bool  $value
     * @return void
     */
    public static function fakeDnsLookups($value = true)
    {}

    /**
     * Flush the validator's global state.
     *
     * @return void
     */
    public static function flushState()
    {}

    /**
     * Handle dynamic calls to class methods.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     *
     * @throws \BadMethodCallException
     */
    public function __call($method, $parameters)
    {}
}
