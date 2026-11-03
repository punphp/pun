<?php

namespace Illuminate\Foundation\Http;

use Illuminate\Auth\Access\AuthorizationException;
use Illuminate\Auth\Access\Response;
use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Validation\Factory as ValidationFactory;
use Illuminate\Contracts\Validation\ValidatesWhenResolved;
use Illuminate\Contracts\Validation\Validator;
use Illuminate\Foundation\Http\Attributes\ErrorBag;
use Illuminate\Foundation\Http\Attributes\FailOnUnknownFields;
use Illuminate\Foundation\Http\Attributes\RedirectTo;
use Illuminate\Foundation\Http\Attributes\RedirectToRoute;
use Illuminate\Foundation\Http\Attributes\StopOnFirstFailure;
use Illuminate\Http\Request;
use Illuminate\Routing\Redirector;
use Illuminate\Support\Arr;
use Illuminate\Validation\ValidatesWhenResolvedTrait;
use ReflectionClass;

class FormRequest extends Request implements ValidatesWhenResolved
{
    use ValidatesWhenResolvedTrait;

    /**
     * The container instance.
     *
     * @var \Illuminate\Contracts\Container\Container
     */
    protected $container;

    /**
     * The redirector instance.
     *
     * @var \Illuminate\Routing\Redirector
     */
    protected $redirector;

    /**
     * The URI to redirect to if validation fails.
     *
     * @var string
     */
    protected $redirect;

    /**
     * The route to redirect to if validation fails.
     *
     * @var string
     */
    protected $redirectRoute;

    /**
     * The controller action to redirect to if validation fails.
     *
     * @var string
     */
    protected $redirectAction;

    /**
     * The key to be used for the view error bag.
     *
     * @var string
     */
    protected $errorBag = 'default';

    /**
     * Indicates whether validation should stop after the first rule failure.
     *
     * @var bool
     */
    protected $stopOnFirstFailure = false;

    /**
     * The validator instance.
     *
     * @var \Illuminate\Contracts\Validation\Validator
     */
    protected $validator;

    /**
     * Indicates if unknown fields should be rejected for all form requests.
     *
     * @var bool
     */
    protected static bool $globalFailOnUnknownFields = false;

    /**
     * Get the validator instance for the request.
     *
     * @return \Illuminate\Contracts\Validation\Validator
     */
    protected function getValidatorInstance()
    {}

    /**
     * Configure the form request from class attributes.
     *
     * @return void
     */
    protected function configureFromAttributes()
    {}

    /**
     * Create the default validator instance.
     *
     * @param  \Illuminate\Contracts\Validation\Factory  $factory
     * @return \Illuminate\Contracts\Validation\Validator
     */
    protected function createDefaultValidator(ValidationFactory $factory)
    {}

    /**
     * Get data to be validated from the request.
     *
     * @return array
     */
    public function validationData()
    {}

    /**
     * Get the validation rules for this form request.
     *
     * @return array
     */
    protected function validationRules()
    {}

    /**
     * Determine if fields not present in rules should fail validation.
     *
     * @return bool
     */
    protected function shouldFailOnUnknownFields(): bool
    {}

    /**
     * Validate that no unknown fields were sent as input.
     *
     * @param  \Illuminate\Contracts\Validation\Validator  $validator
     * @return void
     */
    protected function validateNoUnknownFields(Validator $validator): void
    {}

    /**
     * Flatten the given input's keys into dot notation, escaping literal dots within keys.
     *
     * @param  array  $input
     * @param  string  $prefix
     * @return array
     */
    protected function dotInputKeys(array $input, string $prefix = ''): array
    {}

    /**
     * Determine if the given input key is an allowed key based on the validation rules.
     *
     * @param  string  $inputKey
     * @param  array  $allowedKeys
     * @return bool
     */
    protected function isKnownField(string $inputKey, array $allowedKeys): bool
    {}

    /**
     * Handle a failed validation attempt.
     *
     * @param  \Illuminate\Contracts\Validation\Validator  $validator
     * @return void
     *
     * @throws \Illuminate\Validation\ValidationException
     */
    protected function failedValidation(Validator $validator)
    {}

    /**
     * Get the URL to redirect to on a validation error.
     *
     * @return string
     */
    protected function getRedirectUrl()
    {}

    /**
     * Determine if the request passes the authorization check.
     *
     * @return bool
     *
     * @throws \Illuminate\Auth\Access\AuthorizationException
     */
    protected function passesAuthorization()
    {}

    /**
     * Handle a failed authorization attempt.
     *
     * @return void
     *
     * @throws \Illuminate\Auth\Access\AuthorizationException
     */
    protected function failedAuthorization()
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
     * Get the validated data from the request.
     *
     * @param  array|int|string|null  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function validated($key = null, $default = null)
    {}

    /**
     * Get custom messages for validator errors.
     *
     * @return array<string, string>
     */
    public function messages()
    {}

    /**
     * Get custom attributes for validator errors.
     *
     * @return array<string, string>
     */
    public function attributes()
    {}

    /**
     * Get the nearest class in the request's hierarchy that applies any of the given attributes.
     *
     * @param  array<int, class-string>  $attributes
     * @param  array<int, string>  $properties
     * @return \ReflectionClass<\Illuminate\Foundation\Http\FormRequest>|null
     */
    protected function nearestClassWithAttribute(array $attributes, array $properties = [])
    {}

    /**
     * Enable or disable unknown-field rejection globally for all form requests.
     *
     * @param  bool  $value
     * @return void
     */
    public static function failOnUnknownFields(bool $value = true): void
    {}

    /**
     * Set the Validator instance.
     *
     * @param  \Illuminate\Contracts\Validation\Validator  $validator
     * @return $this
     */
    public function setValidator(Validator $validator)
    {}

    /**
     * Set the Redirector instance.
     *
     * @param  \Illuminate\Routing\Redirector  $redirector
     * @return $this
     */
    public function setRedirector(Redirector $redirector)
    {}

    /**
     * Set the container implementation.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return $this
     */
    public function setContainer(Container $container)
    {}

    /**
     * Flush the global state of the form request.
     *
     * @return void
     */
    public static function flushState(): void
    {}
}
