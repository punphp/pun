<?php

namespace Illuminate\Database\Eloquent\Concerns;

use BackedEnum;
use Brick\Math\BigDecimal;
use Brick\Math\Exception\MathException as BrickMathException;
use Brick\Math\RoundingMode;
use Carbon\CarbonImmutable;
use Carbon\CarbonInterface;
use DateTimeImmutable;
use DateTimeInterface;
use Illuminate\Contracts\Database\Eloquent\Castable;
use Illuminate\Contracts\Database\Eloquent\CastsInboundAttributes;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Database\Eloquent\Attributes\Appends;
use Illuminate\Database\Eloquent\Attributes\DateFormat;
use Illuminate\Database\Eloquent\Attributes\Table;
use Illuminate\Database\Eloquent\Casts\AsArrayObject;
use Illuminate\Database\Eloquent\Casts\AsCollection;
use Illuminate\Database\Eloquent\Casts\AsEncryptedArrayObject;
use Illuminate\Database\Eloquent\Casts\AsEncryptedCollection;
use Illuminate\Database\Eloquent\Casts\AsEnumArrayObject;
use Illuminate\Database\Eloquent\Casts\AsEnumCollection;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Casts\Json;
use Illuminate\Database\Eloquent\InvalidCastException;
use Illuminate\Database\Eloquent\JsonEncodingException;
use Illuminate\Database\Eloquent\MissingAttributeException;
use Illuminate\Database\Eloquent\Relations\Relation;
use Illuminate\Database\LazyLoadingViolationException;
use Illuminate\Support\Arr;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Collection as BaseCollection;
use Illuminate\Support\Exceptions\MathException;
use Illuminate\Support\Facades\Crypt;
use Illuminate\Support\Facades\Date;
use Illuminate\Support\Facades\Hash;
use Illuminate\Support\Str;
use InvalidArgumentException;
use LogicException;
use ReflectionClass;
use ReflectionMethod;
use ReflectionNamedType;
use RuntimeException;
use Stringable;
use ValueError;

use function Illuminate\Support\enum_value;

trait HasAttributes
{
    /**
     * The model's attributes.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [];

    /**
     * The model attribute's original state.
     *
     * @var array<string, mixed>
     */
    protected $original = [];

    /**
     * The changed model attributes.
     *
     * @var array<string, mixed>
     */
    protected $changes = [];

    /**
     * The previous state of the changed model attributes.
     *
     * @var array<string, mixed>
     */
    protected $previous = [];

    /**
     * The attributes that should be cast.
     *
     * @var array
     */
    protected $casts = [];

    /**
     * The attributes that have been cast using custom classes.
     *
     * @var array
     */
    protected $classCastCache = [];

    /**
     * The attributes that have been cast using "Attribute" return type mutators.
     *
     * @var array
     */
    protected $attributeCastCache = [];

    /**
     * The built-in, primitive cast types supported by Eloquent.
     *
     * @var string[]
     */
    protected static $primitiveCastTypes = [
        'array',
        'bool',
        'boolean',
        'collection',
        'custom_datetime',
        'date',
        'datetime',
        'decimal',
        'double',
        'encrypted',
        'encrypted:array',
        'encrypted:collection',
        'encrypted:json',
        'encrypted:object',
        'float',
        'hashed',
        'immutable_date',
        'immutable_datetime',
        'immutable_custom_datetime',
        'int',
        'integer',
        'json',
        'json:unicode',
        'object',
        'real',
        'string',
        'timestamp',
    ];

    /**
     * The storage format of the model's date columns.
     *
     * @var string|null
     */
    protected $dateFormat;

    /**
     * The accessors to append to the model's array form.
     *
     * @var array
     */
    protected $appends = [];

    /**
     * Indicates whether attributes are snake cased on arrays.
     *
     * @var bool
     */
    public static $snakeAttributes = true;

    /**
     * The cache of the mutated attributes for each class.
     *
     * @var array
     */
    protected static $mutatorCache = [];

    /**
     * The cache of the "Attribute" return type marked mutated attributes for each class.
     *
     * @var array
     */
    protected static $attributeMutatorCache = [];

    /**
     * The cache of the "Attribute" return type marked mutated, gettable attributes for each class.
     *
     * @var array
     */
    protected static $getAttributeMutatorCache = [];

    /**
     * The cache of the "Attribute" return type marked mutated, settable attributes for each class.
     *
     * @var array
     */
    protected static $setAttributeMutatorCache = [];

    /**
     * The cache of the converted cast types.
     *
     * @var array
     */
    protected static $castTypeCache = [];

    /**
     * The encrypter instance that is used to encrypt attributes.
     *
     * @var \Illuminate\Contracts\Encryption\Encrypter|null
     */
    public static $encrypter;

    /**
     * Initialize the trait.
     *
     * @return void
     */
    protected function initializeHasAttributes()
    {}

    /**
     * Convert the model's attributes to an array.
     *
     * @return array<string, mixed>
     */
    public function attributesToArray()
    {}

    /**
     * Add the date attributes to the attributes array.
     *
     * @param  array<string, mixed>  $attributes
     * @return array<string, mixed>
     */
    protected function addDateAttributesToArray(array $attributes)
    {}

    /**
     * Add the mutated attributes to the attributes array.
     *
     * @param  array<string, mixed>  $attributes
     * @param  array<string, mixed>  $mutatedAttributes
     * @return array<string, mixed>
     */
    protected function addMutatedAttributesToArray(array $attributes, array $mutatedAttributes)
    {}

    /**
     * Add the casted attributes to the attributes array.
     *
     * @param  array<string, mixed>  $attributes
     * @param  array<string, mixed>  $mutatedAttributes
     * @return array<string, mixed>
     */
    protected function addCastAttributesToArray(array $attributes, array $mutatedAttributes)
    {}

    /**
     * Get an attribute array of all arrayable attributes.
     *
     * @return array<string, mixed>
     */
    protected function getArrayableAttributes()
    {}

    /**
     * Get all of the appendable values that are arrayable.
     *
     * @return array
     */
    protected function getArrayableAppends()
    {}

    /**
     * Get the model's relationships in array form.
     *
     * @return array
     */
    public function relationsToArray()
    {}

    /**
     * Get an attribute array of all arrayable relations.
     *
     * @return array
     */
    protected function getArrayableRelations()
    {}

    /**
     * Get an attribute array of all arrayable values.
     *
     * @param  array  $values
     * @return array
     */
    protected function getArrayableItems(array $values)
    {}

    /**
     * Determine whether an attribute exists on the model.
     *
     * @param  string  $key
     * @return bool
     */
    public function hasAttribute($key)
    {}

    /**
     * Get an attribute from the model.
     *
     * @param  string  $key
     * @return mixed
     */
    public function getAttribute($key)
    {}

    /**
     * Either throw a missing attribute exception or return null depending on Eloquent's configuration.
     *
     * @param  string  $key
     * @return null
     *
     * @throws \Illuminate\Database\Eloquent\MissingAttributeException
     */
    protected function throwMissingAttributeExceptionIfApplicable($key)
    {}

    /**
     * Get a plain attribute (not a relationship).
     *
     * @param  string  $key
     * @return mixed
     */
    public function getAttributeValue($key)
    {}

    /**
     * Get an attribute from the $attributes array.
     *
     * @param  string  $key
     * @return mixed
     */
    protected function getAttributeFromArray($key)
    {}

    /**
     * Get a relationship.
     *
     * @param  string  $key
     * @return mixed
     */
    public function getRelationValue($key)
    {}

    /**
     * Determine if the given key is a relationship method on the model.
     *
     * @param  string  $key
     * @return bool
     */
    public function isRelation($key)
    {}

    /**
     * Handle a lazy loading violation.
     *
     * @param  string  $key
     * @return mixed
     *
     * @throws \Illuminate\Database\LazyLoadingViolationException
     */
    protected function handleLazyLoadingViolation($key)
    {}

    /**
     * Get a relationship value from a method.
     *
     * @param  string  $method
     * @return mixed
     *
     * @throws \LogicException
     */
    protected function getRelationshipFromMethod($method)
    {}

    /**
     * Determine if a get mutator exists for an attribute.
     *
     * @param  string  $key
     * @return bool
     */
    public function hasGetMutator($key)
    {}

    /**
     * Determine if a "Attribute" return type marked mutator exists for an attribute.
     *
     * @param  string  $key
     * @return bool
     */
    public function hasAttributeMutator($key)
    {}

    /**
     * Determine if a "Attribute" return type marked get mutator exists for an attribute.
     *
     * @param  string  $key
     * @return bool
     */
    public function hasAttributeGetMutator($key)
    {}

    /**
     * Determine if any get mutator exists for an attribute.
     *
     * @param  string  $key
     * @return bool
     */
    public function hasAnyGetMutator($key)
    {}

    /**
     * Get the value of an attribute using its mutator.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    protected function mutateAttribute($key, $value)
    {}

    /**
     * Get the value of an "Attribute" return type marked attribute using its mutator.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    protected function mutateAttributeMarkedAttribute($key, $value)
    {}

    /**
     * Get the value of an attribute using its mutator for array conversion.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    protected function mutateAttributeForArray($key, $value)
    {}

    /**
     * Merge new casts with existing casts on the model.
     *
     * @param  array  $casts
     * @return $this
     */
    public function mergeCasts($casts)
    {}

    /**
     * Ensure that the given casts are strings.
     *
     * @param  array  $casts
     * @return array
     *
     * @throws \InvalidArgumentException
     */
    protected function ensureCastsAreStringValues($casts)
    {}

    /**
     * Cast an attribute to a native PHP type.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    protected function castAttribute($key, $value)
    {}

    /**
     * Cast the given attribute using a custom cast class.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    protected function getClassCastableAttributeValue($key, $value)
    {}

    /**
     * Cast the given attribute to an enum.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    protected function getEnumCastableAttributeValue($key, $value)
    {}

    /**
     * Get the type of cast for a model attribute.
     *
     * @param  string  $key
     * @return string
     */
    protected function getCastType($key)
    {}

    /**
     * Increment or decrement the given attribute using the custom cast class.
     *
     * @param  string  $method
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    protected function deviateClassCastableAttribute($method, $key, $value)
    {}

    /**
     * Serialize the given attribute using the custom cast class.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    protected function serializeClassCastableAttribute($key, $value)
    {}

    /**
     * Compare two values for the given attribute using the custom cast class.
     *
     * @param  string  $key
     * @param  mixed  $original
     * @param  mixed  $value
     * @return bool
     */
    protected function compareClassCastableAttribute($key, $original, $value)
    {}

    /**
     * Determine if the cast type is a custom date time cast.
     *
     * @param  string  $cast
     * @return bool
     */
    protected function isCustomDateTimeCast($cast)
    {}

    /**
     * Determine if the cast type is an immutable custom date time cast.
     *
     * @param  string  $cast
     * @return bool
     */
    protected function isImmutableCustomDateTimeCast($cast)
    {}

    /**
     * Determine if the cast type is a decimal cast.
     *
     * @param  string  $cast
     * @return bool
     */
    protected function isDecimalCast($cast)
    {}

    /**
     * Set a given attribute on the model.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    public function setAttribute($key, $value)
    {}

    /**
     * Determine if a set mutator exists for an attribute.
     *
     * @param  string  $key
     * @return bool
     */
    public function hasSetMutator($key)
    {}

    /**
     * Determine if an "Attribute" return type marked set mutator exists for an attribute.
     *
     * @param  string  $key
     * @return bool
     */
    public function hasAttributeSetMutator($key)
    {}

    /**
     * Set the value of an attribute using its mutator.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    protected function setMutatedAttributeValue($key, $value)
    {}

    /**
     * Set the value of a "Attribute" return type marked attribute using its mutator.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    protected function setAttributeMarkedMutatedAttributeValue($key, $value)
    {}

    /**
     * Determine if the given attribute is a date or date castable.
     *
     * @param  string  $key
     * @return bool
     */
    protected function isDateAttribute($key)
    {}

    /**
     * Set a given JSON attribute on the model.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return $this
     */
    public function fillJsonAttribute($key, $value)
    {}

    /**
     * Set the value of a class castable attribute.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return void
     */
    protected function setClassCastableAttribute($key, $value)
    {}

    /**
     * Set the value of an enum castable attribute.
     *
     * @param  string  $key
     * @param  \UnitEnum|string|int|null  $value
     * @return void
     */
    protected function setEnumCastableAttribute($key, $value)
    {}

    /**
     * Get an enum case instance from a given class and value.
     *
     * @param  string  $enumClass
     * @param  string|int  $value
     * @return \UnitEnum
     */
    protected function getEnumCaseFromValue($enumClass, $value)
    {}

    /**
     * Get the storable value from the given enum.
     *
     * @param  string  $expectedEnum
     * @param  \UnitEnum  $value
     * @return string|int
     *
     * @throws \ValueError
     */
    protected function getStorableEnumValue($expectedEnum, $value)
    {}

    /**
     * Get an array attribute with the given key and value set.
     *
     * @param  string  $path
     * @param  string  $key
     * @param  mixed  $value
     * @return array
     */
    protected function getArrayAttributeWithValue($path, $key, $value)
    {}

    /**
     * Get an array attribute or return an empty array if it is not set.
     *
     * @param  string  $key
     * @return array
     */
    protected function getArrayAttributeByKey($key)
    {}

    /**
     * Cast the given attribute to JSON.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return string
     *
     * @throws \Illuminate\Database\Eloquent\JsonEncodingException
     */
    protected function castAttributeAsJson($key, $value)
    {}

    /**
     * Get the JSON casting flags for the given attribute.
     *
     * @param  string  $key
     * @return int
     */
    protected function getJsonCastFlags($key)
    {}

    /**
     * Encode the given value as JSON.
     *
     * @param  mixed  $value
     * @param  int  $flags
     * @return string
     */
    protected function asJson($value, $flags = 0)
    {}

    /**
     * Decode the given JSON back into an array or object.
     *
     * @param  string|null  $value
     * @param  bool  $asObject
     * @return mixed
     */
    public function fromJson($value, $asObject = false)
    {}

    /**
     * Decrypt the given encrypted string.
     *
     * @param  string  $value
     * @return mixed
     */
    public function fromEncryptedString($value)
    {}

    /**
     * Cast the given attribute to an encrypted string.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return string
     */
    protected function castAttributeAsEncryptedString($key, #[\SensitiveParameter] $value)
    {}

    /**
     * Set the encrypter instance that will be used to encrypt attributes.
     *
     * @param  \Illuminate\Contracts\Encryption\Encrypter|null  $encrypter
     * @return void
     */
    public static function encryptUsing($encrypter)
    {}

    /**
     * Get the current encrypter being used by the model.
     *
     * @return \Illuminate\Contracts\Encryption\Encrypter
     */
    public static function currentEncrypter()
    {}

    /**
     * Cast the given attribute to a hashed string.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return string|null
     *
     * @throws \RuntimeException
     */
    protected function castAttributeAsHashedString($key, #[\SensitiveParameter] $value)
    {}

    /**
     * Decode the given float.
     *
     * @param  mixed  $value
     * @return mixed
     */
    public function fromFloat($value)
    {}

    /**
     * Return a decimal as string.
     *
     * @param  float|string  $value
     * @param  int  $decimals
     * @return string
     *
     * @throws \Illuminate\Support\Exceptions\MathException
     */
    protected function asDecimal($value, $decimals)
    {}

    /**
     * Return a timestamp as DateTime object with time set to 00:00:00.
     *
     * @param  mixed  $value
     * @return \Illuminate\Support\Carbon
     */
    protected function asDate($value)
    {}

    /**
     * Return a timestamp as DateTime object.
     *
     * @param  mixed  $value
     * @return \Illuminate\Support\Carbon
     */
    protected function asDateTime($value)
    {}

    /**
     * Determine if the given value is a standard date format.
     *
     * @param  string  $value
     * @return bool
     */
    protected function isStandardDateFormat($value)
    {}

    /**
     * Convert a DateTime to a storable string.
     *
     * @param  mixed  $value
     * @return string|null
     */
    public function fromDateTime($value)
    {}

    /**
     * Return a timestamp as unix timestamp.
     *
     * @param  mixed  $value
     * @return int
     */
    protected function asTimestamp($value)
    {}

    /**
     * Prepare a date for array / JSON serialization.
     *
     * @param  \DateTimeInterface  $date
     * @return string
     */
    protected function serializeDate(DateTimeInterface $date)
    {}

    /**
     * Get the attributes that should be converted to dates.
     *
     * @return array<int, string|null>
     */
    public function getDates()
    {}

    /**
     * Get the format for database stored dates.
     *
     * @return string
     */
    public function getDateFormat()
    {}

    /**
     * Set the date format used by the model.
     *
     * @param  string  $format
     * @return $this
     */
    public function setDateFormat($format)
    {}

    /**
     * Determine whether an attribute should be cast to a native type.
     *
     * @param  string  $key
     * @param  array|string|null  $types
     * @return bool
     */
    public function hasCast($key, $types = null)
    {}

    /**
     * Get the attributes that should be cast.
     *
     * @return array
     */
    public function getCasts()
    {}

    /**
     * Get the attributes that should be cast.
     *
     * @return array<string, Stringable|string>
     */
    protected function casts()
    {}

    /**
     * Merge the default values into the model's attributes.
     *
     * @return void
     */
    protected function mergeDefaultAttributes()
    {}

    /**
     * Determine whether a value is Date / DateTime castable for inbound manipulation.
     *
     * @param  string  $key
     * @return bool
     */
    protected function isDateCastable($key)
    {}

    /**
     * Determine whether a value is Date / DateTime custom-castable for inbound manipulation.
     *
     * @param  string  $key
     * @return bool
     */
    protected function isDateCastableWithCustomFormat($key)
    {}

    /**
     * Determine whether a value is JSON castable for inbound manipulation.
     *
     * @param  string  $key
     * @return bool
     */
    protected function isJsonCastable($key)
    {}

    /**
     * Determine whether a value is an encrypted castable for inbound manipulation.
     *
     * @param  string  $key
     * @return bool
     */
    protected function isEncryptedCastable($key)
    {}

    /**
     * Determine whether a value is an encrypted class castable for inbound manipulation.
     *
     * @param  string  $key
     * @return bool
     */
    protected function isEncryptedClassCastable($key)
    {}

    /**
     * Determine if the given key is cast using a custom class.
     *
     * @param  string  $key
     * @return bool
     *
     * @throws \Illuminate\Database\Eloquent\InvalidCastException
     */
    protected function isClassCastable($key)
    {}

    /**
     * Determine if the given key is cast using an enum.
     *
     * @param  string  $key
     * @return bool
     */
    protected function isEnumCastable($key)
    {}

    /**
     * Determine if the key is deviable using a custom class.
     *
     * @param  string  $key
     * @return bool
     *
     * @throws \Illuminate\Database\Eloquent\InvalidCastException
     */
    protected function isClassDeviable($key)
    {}

    /**
     * Determine if the key is serializable using a custom class.
     *
     * @param  string  $key
     * @return bool
     *
     * @throws \Illuminate\Database\Eloquent\InvalidCastException
     */
    protected function isClassSerializable($key)
    {}

    /**
     * Determine if the key is comparable using a custom class.
     *
     * @param  string  $key
     * @return bool
     */
    protected function isClassComparable($key)
    {}

    /**
     * Resolve the custom caster class for a given key.
     *
     * @param  string  $key
     * @return mixed
     */
    protected function resolveCasterClass($key)
    {}

    /**
     * Parse the given caster class, removing any arguments.
     *
     * @param  string  $class
     * @return string
     */
    protected function parseCasterClass($class)
    {}

    /**
     * Merge the cast class and attribute cast attributes back into the model.
     *
     * @return void
     */
    protected function mergeAttributesFromCachedCasts()
    {}

    /**
     * Merge the cast class and attribute cast attribute back into the model.
     *
     * @return void
     */
    protected function mergeAttributeFromCachedCasts(string $key)
    {}

    /**
     * Merge the cast class attributes back into the model.
     *
     * @return void
     */
    protected function mergeAttributesFromClassCasts()
    {}

    /**
     * Merge the cast class attribute back into the model.
     *
     * @return void
     */
    protected function mergeAttributeFromClassCasts(string $key): void
    {}

    /**
     * Merge the cast class attributes back into the model.
     *
     * @return void
     */
    protected function mergeAttributesFromAttributeCasts()
    {}

    /**
     * Merge the cast class attribute back into the model.
     *
     * @return void
     */
    protected function mergeAttributeFromAttributeCasts(string $key): void
    {}

    /**
     * Normalize the response from a custom class caster.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return array
     */
    protected function normalizeCastClassResponse($key, $value)
    {}

    /**
     * Get all of the current attributes on the model.
     *
     * @return array<string, mixed>
     */
    public function getAttributes()
    {}

    /**
     * Get all of the current attributes on the model for an insert operation.
     *
     * @return array
     */
    protected function getAttributesForInsert()
    {}

    /**
     * Set the array of model attributes. No checking is done.
     *
     * @param  array  $attributes
     * @param  bool  $sync
     * @return $this
     */
    public function setRawAttributes(array $attributes, $sync = false)
    {}

    /**
     * Get the model's original attribute values.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return ($key is null ? array<string, mixed> : mixed)
     */
    public function getOriginal($key = null, $default = null)
    {}

    /**
     * Get the model's original attribute values.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return ($key is null ? array<string, mixed> : mixed)
     */
    protected function getOriginalWithoutRewindingModel($key = null, $default = null)
    {}

    /**
     * Get the model's raw original attribute values.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return ($key is null ? array<string, mixed> : mixed)
     */
    public function getRawOriginal($key = null, $default = null)
    {}

    /**
     * Get a subset of the model's attributes.
     *
     * @param  array<string>|mixed  $attributes
     * @return array<string, mixed>
     */
    public function only($attributes)
    {}

    /**
     * Get all attributes except the given ones.
     *
     * @param  array<string>|mixed  $attributes
     * @return array
     */
    public function except($attributes)
    {}

    /**
     * Sync the original attributes with the current.
     *
     * @return $this
     */
    public function syncOriginal()
    {}

    /**
     * Sync a single original attribute with its current value.
     *
     * @param  string  $attribute
     * @return $this
     */
    public function syncOriginalAttribute($attribute)
    {}

    /**
     * Sync multiple original attribute with their current values.
     *
     * @param  array<string>|string  $attributes
     * @return $this
     */
    public function syncOriginalAttributes($attributes)
    {}

    /**
     * Sync the changed attributes.
     *
     * @return $this
     */
    public function syncChanges()
    {}

    /**
     * Determine if the model or any of the given attribute(s) have been modified.
     *
     * @param  array<string>|string|null  $attributes
     * @return bool
     */
    public function isDirty($attributes = null)
    {}

    /**
     * Determine if the model or all the given attribute(s) have remained the same.
     *
     * @param  array<string>|string|null  $attributes
     * @return bool
     */
    public function isClean($attributes = null)
    {}

    /**
     * Discard attribute changes and reset the attributes to their original state.
     *
     * @return $this
     */
    public function discardChanges()
    {}

    /**
     * Determine if the model or any of the given attribute(s) were changed when the model was last saved.
     *
     * @param  array<string>|string|null  $attributes
     * @return bool
     */
    public function wasChanged($attributes = null)
    {}

    /**
     * Determine if any of the given attributes were changed when the model was last saved.
     *
     * @param  array<string>  $changes
     * @param  array<string>|string|null  $attributes
     * @return bool
     */
    protected function hasChanges($changes, $attributes = null)
    {}

    /**
     * Get the attributes that have been changed since the last sync.
     *
     * @return array<string, mixed>
     */
    public function getDirty()
    {}

    /**
     * Get the attributes that have been changed since the last sync for an update operation.
     *
     * @return array<string, mixed>
     */
    protected function getDirtyForUpdate()
    {}

    /**
     * Get the attributes that were changed when the model was last saved.
     *
     * @return array<string, mixed>
     */
    public function getChanges()
    {}

    /**
     * Get the attributes that were previously original before the model was last saved.
     *
     * @return array<string, mixed>
     */
    public function getPrevious()
    {}

    /**
     * Determine if the new and old values for a given key are equivalent.
     *
     * @param  string  $key
     * @return bool
     */
    public function originalIsEquivalent($key)
    {}

    /**
     * Transform a raw model value using mutators, casts, etc.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    protected function transformModelValue($key, $value)
    {}

    /**
     * Append attributes to query when building a query.
     *
     * @param  array<string>|string  $attributes
     * @return $this
     */
    public function append($attributes)
    {}

    /**
     * Get the accessors that are being appended to model arrays.
     *
     * @return array
     */
    public function getAppends()
    {}

    /**
     * Set the accessors to append to model arrays.
     *
     * @param  array  $appends
     * @return $this
     */
    public function setAppends(array $appends)
    {}

    /**
     * Merge new appended attributes with existing appended attributes on the model.
     *
     * @param  array<string>  $appends
     * @return $this
     */
    public function mergeAppends(array $appends)
    {}

    /**
     * Return whether the accessor attribute has been appended.
     *
     * @param  string  $attribute
     * @return bool
     */
    public function hasAppended($attribute)
    {}

    /**
     * Remove all appended properties from the model.
     *
     * @return $this
     */
    public function withoutAppends()
    {}

    /**
     * Get the mutated attributes for a given instance.
     *
     * @return array
     */
    public function getMutatedAttributes()
    {}

    /**
     * Extract and cache all the mutated attributes of a class.
     *
     * @param  object|string  $classOrInstance
     * @return void
     */
    public static function cacheMutatedAttributes($classOrInstance)
    {}

    /**
     * Get all of the attribute mutator methods.
     *
     * @param  mixed  $class
     * @return array
     */
    protected static function getMutatorMethods($class)
    {}

    /**
     * Get all of the "Attribute" return typed attribute mutator methods.
     *
     * @param  mixed  $class
     * @return array
     */
    protected static function getAttributeMarkedMutatorMethods($class)
    {}
}
