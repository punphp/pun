<?php

namespace Illuminate\Validation\Concerns;

use Brick\Math\BigDecimal;
use Brick\Math\BigNumber;
use Brick\Math\Exception\MathException as BrickMathException;
use DateTime;
use DateTimeInterface;
use DateTimeZone;
use Egulias\EmailValidator\EmailValidator;
use Egulias\EmailValidator\Validation\DNSCheckValidation;
use Egulias\EmailValidator\Validation\Extra\SpoofCheckValidation;
use Egulias\EmailValidator\Validation\MultipleValidationWithAnd;
use Egulias\EmailValidator\Validation\NoRFCWarningsValidation;
use Egulias\EmailValidator\Validation\RFCValidation;
use Exception;
use Illuminate\Container\Container;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Exceptions\MathException;
use Illuminate\Support\Facades\Date;
use Illuminate\Support\Str;
use Illuminate\Validation\FakeDnsGetRecordWrapper;
use Illuminate\Validation\Rules\Exists;
use Illuminate\Validation\Rules\Unique;
use Illuminate\Validation\ValidationData;
use InvalidArgumentException;
use Symfony\Component\HttpFoundation\File\File;
use Symfony\Component\HttpFoundation\File\UploadedFile;
use ValueError;

trait ValidatesAttributes
{
    /**
     * Validate that an attribute was "accepted".
     *
     * This validation rule implies the attribute is "required".
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateAccepted($attribute, $value)
    {}

    /**
     * Validate that an attribute was "accepted" when another attribute has a given value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateAcceptedIf($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute was "declined".
     *
     * This validation rule implies the attribute is "required".
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateDeclined($attribute, $value)
    {}

    /**
     * Validate that an attribute was "declined" when another attribute has a given value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateDeclinedIf($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is an active URL.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateActiveUrl($attribute, $value)
    {}

    /**
     * Get the DNS records for the given hostname.
     *
     * @param  string  $hostname
     * @param  int  $type
     * @return array|false
     */
    protected function getDnsRecords($hostname, $type)
    {}

    /**
     * Validate that an attribute is 7 bit ASCII.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateAscii($attribute, $value)
    {}

    /**
     * Validate that an attribute is a valid Base64 string.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateBase64($attribute, $value): bool
    {}

    /**
     * "Break" on first validation fail.
     *
     * Always returns true, just lets us put "bail" in rules.
     *
     * @return bool
     */
    public function validateBail()
    {}

    /**
     * Validate the date is before a given date.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateBefore($attribute, $value, $parameters)
    {}

    /**
     * Validate the date is before or equal a given date.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateBeforeOrEqual($attribute, $value, $parameters)
    {}

    /**
     * Validate the date is after a given date.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateAfter($attribute, $value, $parameters)
    {}

    /**
     * Validate the date is equal or after a given date.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateAfterOrEqual($attribute, $value, $parameters)
    {}

    /**
     * Compare a given date against another using an operator.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @param  string  $operator
     * @return bool
     */
    protected function compareDates($attribute, $value, $parameters, $operator)
    {}

    /**
     * Get the date format for an attribute if it has one.
     *
     * @param  string  $attribute
     * @return string|null
     */
    protected function getDateFormat($attribute)
    {}

    /**
     * Get the date timestamp.
     *
     * @param  mixed  $value
     * @return int|null
     */
    protected function getDateTimestamp($value)
    {}

    /**
     * Given two date/time strings, check that one is after the other.
     *
     * @param  string  $format
     * @param  string  $first
     * @param  string  $second
     * @param  string  $operator
     * @return bool
     */
    protected function checkDateTimeOrder($format, $first, $second, $operator)
    {}

    /**
     * Get a DateTime instance from a string.
     *
     * @param  string  $format
     * @param  mixed  $value
     * @return \DateTime|null
     */
    protected function getDateTimeWithOptionalFormat($format, $value)
    {}

    /**
     * Get a DateTime instance from a string with no format.
     *
     * @param  mixed  $value
     * @return \DateTime|null
     */
    protected function getDateTime($value)
    {}

    /**
     * Validate that an attribute contains only alphabetic characters.
     * If the 'ascii' option is passed, validate that an attribute contains only ascii alphabetic characters.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateAlpha($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute contains only alpha-numeric characters, dashes, and underscores.
     * If the 'ascii' option is passed, validate that an attribute contains only ascii alpha-numeric characters,
     * dashes, and underscores.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateAlphaDash($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute contains only alpha-numeric characters.
     * If the 'ascii' option is passed, validate that an attribute contains only ascii alpha-numeric characters.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateAlphaNum($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is an array.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateArray($attribute, $value, $parameters = [])
    {}

    /**
     * Validate that an array does not contain any keys other than the given keys.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateArrayKeys($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is a list.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateList($attribute, $value)
    {}

    /**
     * Validate that an array has all of the given keys.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateRequiredArrayKeys($attribute, $value, $parameters)
    {}

    /**
     * Validate the size of an attribute is between a set of values.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateBetween($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is a boolean.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array{0?: 'strict'}  $parameters
     * @return bool
     */
    public function validateBoolean($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute has a matching confirmation.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array{0?: string}  $parameters
     * @return bool
     */
    public function validateConfirmed($attribute, $value, $parameters)
    {}

    /**
     * Validate an attribute contains a list of values.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateContains($attribute, $value, $parameters)
    {}

    /**
     * Validate an attribute does not contain a list of values.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateDoesntContain($attribute, $value, $parameters)
    {}

    /**
     * Validate that the password of the currently authenticated user matches the given value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    protected function validateCurrentPassword($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is a valid date.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateDate($attribute, $value)
    {}

    /**
     * Validate that an attribute matches a date format.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateDateFormat($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is equal to another date.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateDateEquals($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute has a given number of decimal places.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateDecimal($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is different from another attribute.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateDifferent($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute has a given number of digits.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateDigits($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is between a given number of digits.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateDigitsBetween($attribute, $value, $parameters)
    {}

    /**
     * Validate the dimensions of an image matches the given values.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateDimensions($attribute, $value, $parameters)
    {}

    /**
     * Test if the given width and height fail any conditions.
     *
     * @param  array<string,string>  $parameters
     * @param  int  $width
     * @param  int  $height
     * @return bool
     */
    protected function failsBasicDimensionChecks($parameters, $width, $height)
    {}

    /**
     * Determine if the given parameters fail a dimension ratio check.
     *
     * @param  array<string,string>  $parameters
     * @param  int  $width
     * @param  int  $height
     * @return bool
     */
    protected function failsRatioCheck($parameters, $width, $height)
    {}

    /**
     * Determine if the given parameters fail a dimension minimum ratio check.
     *
     * @param  array<string,string>  $parameters
     * @param  int  $width
     * @param  int  $height
     * @return bool
     */
    private function failsMinRatioCheck($parameters, $width, $height)
    {}

    /**
     * Determine if the given parameters fail a dimension maximum ratio check.
     *
     * @param  array<string,string>  $parameters
     * @param  int  $width
     * @param  int  $height
     * @return bool
     */
    private function failsMaxRatioCheck($parameters, $width, $height)
    {}

    /**
     * Validate an attribute is unique among other values.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateDistinct($attribute, $value, $parameters)
    {}

    /**
     * Get the values to distinct between.
     *
     * @param  string  $attribute
     * @return array
     */
    protected function getDistinctValues($attribute)
    {}

    /**
     * Extract the distinct values from the data.
     *
     * @param  string  $attribute
     * @return array
     */
    protected function extractDistinctValues($attribute)
    {}

    /**
     * Validate that an attribute is a valid e-mail address.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateEmail($attribute, $value, $parameters)
    {}

    /**
     * Validate the encoding of an attribute.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     *
     * @throws \InvalidArgumentException
     */
    public function validateEncoding($attribute, $value, $parameters)
    {}

    /**
     * Validate the existence of an attribute value in a database table.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateExists($attribute, $value, $parameters)
    {}

    /**
     * Get the number of records that exist in storage.
     *
     * @param  mixed  $connection
     * @param  string  $table
     * @param  string  $column
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return int
     */
    protected function getExistCount($connection, $table, $column, $value, $parameters)
    {}

    /**
     * Validate the uniqueness of an attribute value on a given database table.
     *
     * If a database column is not specified, the attribute will be used.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateUnique($attribute, $value, $parameters)
    {}

    /**
     * Get the excluded ID column and value for the unique rule.
     *
     * @param  string|null  $idColumn
     * @param  array<int, int|string>  $parameters
     * @return array
     */
    protected function getUniqueIds($idColumn, $parameters)
    {}

    /**
     * Prepare the given ID for querying.
     *
     * @param  mixed  $id
     * @return int
     */
    protected function prepareUniqueId($id)
    {}

    /**
     * Get the extra conditions for a unique rule.
     *
     * @param  array<int, int|string>  $parameters
     * @return array
     */
    protected function getUniqueExtra($parameters)
    {}

    /**
     * Parse the connection / table for the unique / exists rules.
     *
     * @param  string  $table
     * @return array
     */
    public function parseTable($table)
    {}

    /**
     * Get the column name for an exists / unique query.
     *
     * @param  array<int, int|string>  $parameters
     * @param  string  $attribute
     * @return int|string
     */
    public function getQueryColumn($parameters, $attribute)
    {}

    /**
     * Guess the database column from the given attribute name.
     *
     * @param  string  $attribute
     * @return string
     */
    public function guessColumnForQuery($attribute)
    {}

    /**
     * Get the extra conditions for a unique / exists rule.
     *
     * @param  array  $segments
     * @return array
     */
    protected function getExtraConditions(array $segments)
    {}

    /**
     * Validate the extension of a file upload attribute is in a set of defined extensions.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateExtensions($attribute, $value, $parameters)
    {}

    /**
     * Validate the given value is a valid file.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateFile($attribute, $value)
    {}

    /**
     * Validate the given attribute is filled if it is present.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateFilled($attribute, $value)
    {}

    /**
     * Validate that an attribute is greater than another attribute.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateGt($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is less than another attribute.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateLt($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is greater than or equal another attribute.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateGte($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is less than or equal another attribute.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateLte($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is lowercase.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateLowercase($attribute, $value)
    {}

    /**
     * Validate that an attribute is uppercase.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateUppercase($attribute, $value)
    {}

    /**
     * Validate that an attribute is a valid HEX color.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateHexColor($attribute, $value)
    {}

    /**
     * Validate the MIME type of a file is an image MIME type.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, string>  $parameters
     * @return bool
     */
    public function validateImage($attribute, $value, $parameters = [])
    {}

    /**
     * Validate an attribute is contained within a list of values.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateIn($attribute, $value, $parameters)
    {}

    /**
     * Validate that the values of an attribute are in another attribute.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateInArray($attribute, $value, $parameters)
    {}

    /**
     * Validate that an array has at least one of the given keys.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateInArrayKeys($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is an integer.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array{0?: 'strict'}  $parameters
     * @return bool
     */
    public function validateInteger($attribute, $value, array $parameters = [])
    {}

    /**
     * Validate that an attribute is a valid IP.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateIp($attribute, $value)
    {}

    /**
     * Validate that an attribute is a valid IPv4.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateIpv4($attribute, $value)
    {}

    /**
     * Validate that an attribute is a valid IPv6.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateIpv6($attribute, $value)
    {}

    /**
     * Validate that an attribute is a valid MAC address.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateMacAddress($attribute, $value)
    {}

    /**
     * Validate the attribute is a valid JSON string.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateJson($attribute, $value)
    {}

    /**
     * Validate the size of an attribute is less than or equal to a maximum value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateMax($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute has a maximum number of digits.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateMaxDigits($attribute, $value, $parameters)
    {}

    /**
     * Validate the guessed extension of a file upload is in a set of file extensions.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateMimes($attribute, $value, $parameters)
    {}

    /**
     * Validate the MIME type of a file upload attribute is in a set of MIME types.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateMimetypes($attribute, $value, $parameters)
    {}

    /**
     * Check if PHP uploads are explicitly allowed.
     *
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    protected function shouldBlockPhpUpload($value, $parameters)
    {}

    /**
     * Validate the size of an attribute is greater than or equal to a minimum value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateMin($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute has a minimum number of digits.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateMinDigits($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is missing.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateMissing($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is missing when another attribute has a given value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateMissingIf($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is missing unless another attribute has a given value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateMissingUnless($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is missing when any given attribute is present.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateMissingWith($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is missing when all given attributes are present.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateMissingWithAll($attribute, $value, $parameters)
    {}

    /**
     * Validate the value of an attribute is a multiple of a given value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     *
     * @throws \Illuminate\Support\Exceptions\MathException
     */
    public function validateMultipleOf($attribute, $value, $parameters)
    {}

    /**
     * "Indicate" validation should pass if value is null.
     *
     * Always returns true, just lets us put "nullable" in rules.
     *
     * @return bool
     */
    public function validateNullable()
    {}

    /**
     * Validate an attribute is not contained within a list of values.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateNotIn($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is numeric.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array{0?: 'strict'}  $parameters
     * @return bool
     */
    public function validateNumeric($attribute, $value, array $parameters)
    {}

    /**
     * Validate that an attribute exists even if not filled.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validatePresent($attribute, $value)
    {}

    /**
     * Validate that an attribute is present when another attribute has a given value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validatePresentIf($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is present unless another attribute has a given value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validatePresentUnless($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is present when any given attribute is present.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validatePresentWith($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is present when all given attributes are present.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validatePresentWithAll($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute passes a regular expression check.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateRegex($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute does not pass a regular expression check.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateNotRegex($attribute, $value, $parameters)
    {}

    /**
     * Validate that a required attribute exists.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateRequired($attribute, $value)
    {}

    /**
     * Validate that an attribute exists when another attribute has a given value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateRequiredIf($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute exists when another attribute was "accepted".
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateRequiredIfAccepted($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute exists when another attribute was "declined".
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateRequiredIfDeclined($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute does not exist or is an empty string.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateProhibited($attribute, $value)
    {}

    /**
     * Validate that an attribute does not exist when another attribute has a given value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateProhibitedIf($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute does not exist when another attribute was "accepted".
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateProhibitedIfAccepted($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute does not exist when another attribute was "declined".
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateProhibitedIfDeclined($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute does not exist unless another attribute has a given value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateProhibitedUnless($attribute, $value, $parameters)
    {}

    /**
     * Validate that other attributes do not exist when this attribute exists.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateProhibits($attribute, $value, $parameters)
    {}

    /**
     * Indicate that an attribute is excluded.
     *
     * @return bool
     */
    public function validateExclude()
    {}

    /**
     * Indicate that an attribute should be excluded when another attribute has a given value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateExcludeIf($attribute, $value, $parameters)
    {}

    /**
     * Indicate that an attribute should be excluded when another attribute does not have a given value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateExcludeUnless($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute exists when another attribute does not have a given value.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateRequiredUnless($attribute, $value, $parameters)
    {}

    /**
     * Indicate that an attribute should be excluded when another attribute presents.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateExcludeWith($attribute, $value, $parameters)
    {}

    /**
     * Indicate that an attribute should be excluded when another attribute is missing.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateExcludeWithout($attribute, $value, $parameters)
    {}

    /**
     * Prepare the values and the other value for validation.
     *
     * @param  array<int, int|string>  $parameters
     * @return array
     */
    public function parseDependentRuleParameters($parameters)
    {}

    /**
     * Check if parameter should be converted to boolean.
     *
     * @param  string  $parameter
     * @return bool
     */
    protected function shouldConvertToBoolean($parameter)
    {}

    /**
     * Convert the given values to boolean if they are string "true" / "false".
     *
     * @param  array  $values
     * @return array
     */
    protected function convertValuesToBoolean($values)
    {}

    /**
     * Convert the given values to null if they are string "null".
     *
     * @param  array  $values
     * @return array
     */
    protected function convertValuesToNull($values)
    {}

    /**
     * Validate that an attribute exists when any other attribute exists.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateRequiredWith($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute exists when all other attributes exist.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateRequiredWithAll($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute exists when another attribute does not.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateRequiredWithout($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute exists when all other attributes do not.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $parameters
     * @return bool
     */
    public function validateRequiredWithoutAll($attribute, $value, $parameters)
    {}

    /**
     * Determine if any of the given attributes fail the required test.
     *
     * @param  array  $attributes
     * @return bool
     */
    protected function anyFailingRequired(array $attributes)
    {}

    /**
     * Determine if all of the given attributes fail the required test.
     *
     * @param  array  $attributes
     * @return bool
     */
    protected function allFailingRequired(array $attributes)
    {}

    /**
     * Validate that two attributes match.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateSame($attribute, $value, $parameters)
    {}

    /**
     * Validate the size of an attribute.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateSize($attribute, $value, $parameters)
    {}

    /**
     * "Validate" optional attributes.
     *
     * Always returns true, just lets us put sometimes in rules.
     *
     * @return bool
     */
    public function validateSometimes()
    {}

    /**
     * Validate the attribute starts with a given substring.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateStartsWith($attribute, $value, $parameters)
    {}

    /**
     * Validate the attribute does not start with a given substring.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateDoesntStartWith($attribute, $value, $parameters)
    {}

    /**
     * Validate the attribute ends with a given substring.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateEndsWith($attribute, $value, $parameters)
    {}

    /**
     * Validate the attribute does not end with a given substring.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int|string>  $parameters
     * @return bool
     */
    public function validateDoesntEndWith($attribute, $value, $parameters)
    {}

    /**
     * Validate that an attribute is a string.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateString($attribute, $value)
    {}

    /**
     * Validate that an attribute is a valid timezone.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<string, null|string>  $parameters
     * @return bool
     */
    public function validateTimezone($attribute, $value, $parameters = [])
    {}

    /**
     * Validate that an attribute is a valid URL.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, string>  $parameters
     * @return bool
     */
    public function validateUrl($attribute, $value, $parameters = [])
    {}

    /**
     * Validate that an attribute is a valid ULID.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function validateUlid($attribute, $value)
    {}

    /**
     * Validate that an attribute is a valid UUID.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  array<int, int<0, 8>|'max'>  $parameters
     * @return bool
     */
    public function validateUuid($attribute, $value, $parameters)
    {}

    /**
     * Get the size of an attribute.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return int|string
     */
    protected function getSize($attribute, $value)
    {}

    /**
     * Check that the given value is a valid file instance.
     *
     * @param  mixed  $value
     * @return bool
     */
    public function isValidFileInstance($value)
    {}

    /**
     * Determine if a comparison passes between the given values.
     *
     * @param  mixed  $first
     * @param  mixed  $second
     * @param  string  $operator
     * @return bool
     *
     * @throws \InvalidArgumentException
     */
    protected function compare($first, $second, $operator)
    {}

    /**
     * Parse named parameters to $key => $value items.
     *
     * @param  array<int, int|string>  $parameters
     * @return array
     */
    public function parseNamedParameters($parameters)
    {}

    /**
     * Require a certain number of parameters to be present.
     *
     * @param  int  $count
     * @param  array<int, int|string>  $parameters
     * @param  string  $rule
     * @return void
     *
     * @throws \InvalidArgumentException
     */
    public function requireParameterCount($count, $parameters, $rule)
    {}

    /**
     * Check if the parameters are of the same type.
     *
     * @param  mixed  $first
     * @param  mixed  $second
     * @return bool
     */
    protected function isSameType($first, $second)
    {}

    /**
     * Adds the existing rule to the numericRules array if the attribute's value is numeric.
     *
     * @param  string  $attribute
     * @param  string  $rule
     * @return void
     */
    protected function shouldBeNumeric($attribute, $rule)
    {}

    /**
     * Trim the value if it is a string.
     *
     * @param  mixed  $value
     * @return string
     */
    protected function trim($value)
    {}

    /**
     * Ensure the exponent is within the allowed range.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return mixed
     *
     * @throws \Illuminate\Support\Exceptions\MathException
     */
    protected function ensureExponentWithinAllowedRange($attribute, $value)
    {}
}
