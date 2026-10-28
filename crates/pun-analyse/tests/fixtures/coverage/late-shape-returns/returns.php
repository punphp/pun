<?php declare(strict_types=1);

namespace LateShapeReturns;

/**
 * @template TKey of string
 * @param TKey $key
 * @return array{TKey: int}
 */
function requiredString($key) {}

/**
 * @template TKey of string
 * @param TKey $key
 * @return array{TKey?: int}
 */
function optionalString($key) {}

/**
 * @template TKey of int|string
 * @param TKey $key
 * @return array{TKey: int}
 */
function requiredUnion($key) {}

/**
 * @template TKey of int|string
 * @param TKey $key
 * @return array{TKey?: int}
 */
function optionalUnion($key) {}

/**
 * @template TKey of \stdClass
 * @param TKey $key
 * @return array{TKey: int}
 */
function invalidObject($key) {}

/**
 * @template TKey of array<string>
 * @param TKey $key
 * @return array{TKey: int}
 */
function invalidArray($key) {}

/**
 * @template TKey of string|\stdClass
 * @param TKey $key
 * @return array{TKey: int}
 */
function invalidUnion($key) {}

/**
 * @template TKey of string|array<string>
 * @param TKey $key
 * @return array{TKey?: int}
 */
function invalidOptionalUnion($key) {}

/**
 * @template TKey of string
 * @param TKey $key
 * @return array{nested: array{TKey: int}}
 */
function nestedRequired($key) {}

/**
 * @template TKey of string
 * @param TKey $key
 * @return array{nested?: array{TKey?: int}}
 */
function nestedOptional($key) {}

/**
 * @template TKey of \stdClass
 * @param TKey $key
 * @return array{nested: array{TKey: int}}
 */
function nestedInvalid($key) {}

/**
 * @template TKey of string
 * @template TValue of \stdClass|string
 * @param TKey $key
 * @param TValue $value
 * @return array{TKey: TValue}
 */
function valueTemplate($key, $value) {}

/**
 * @template TKey of string
 * @param TKey $key
 * @return array{TKey: int}
 */
function emptyReturn($key) { return; }

/**
 * @template TKey of string
 * @param TKey $key
 * @return array{TKey?: int}
 */
function optionalEmptyReturn($key) { return; }

/**
 * @template TKey of string
 * @param TKey $key
 * @return array{TKey?: int}
 */
function wrongValue($key) { return 'wrong'; }

/**
 * @template TKey of string
 * @param TKey $key
 * @return array{TKey: int}
 */
function requiredWrongArray($key) { return ['key' => 'wrong']; }

/**
 * @template TKey of string
 * @param TKey $key
 * @return array{TKey?: int}
 */
function optionalEmptyArray($key) { return []; }

/**
 * @template TKey of string
 * @param TKey $key
 * @return array{TKey: int}
 */
function validArray($key) { return ['key' => 1]; }

/**
 * @template TKey of string
 * @param TKey $key
 * @return array{nested: array{TKey: int}}
 */
function nestedEmptyReturn($key) { return; }

/**
 * @template TKey of string
 * @template TValue of \stdClass|string
 * @param TKey $key
 * @param TValue $value
 * @return array{TKey: TValue}
 */
function wrongTemplateValue($key, $value) { return ['key' => 1]; }

/**
 * @template TKey of string
 * @param TKey $key
 * @return array{TKey: int}
 */
function conditionalReturn($key, bool $flag) {
    if ($flag) {
        return ['key' => 1];
    }
}

class Methods
{
    /**
     * @template TKey of string
     * @param TKey $key
     * @return array{TKey: int}
     */
    public function missing($key) {}

    /**
     * @template TKey of \stdClass
     * @param TKey $key
     * @return array{TKey?: int}
     */
    public function invalidOptional($key) {}

    /**
     * @template TKey of string
     * @param TKey $key
     * @return array{TKey: int}
     */
    public function emptyReturn($key) { return; }

    /**
     * @template TKey of string
     * @param TKey $key
     * @return array{TKey: int}
     */
    public function wrongValue($key) { return ['key' => 'wrong']; }

    /**
     * @template TKey of string
     * @param TKey $key
     * @return array{TKey?: int}
     */
    public function validOptional($key) { return []; }

    /**
     * @template TKey of string
     * @param TKey $key
     * @return array{TKey: static}
     */
    public function staticValue($key) {}

    /**
     * @template TKey of string
     * @param TKey $key
     * @return ($flag is true ? array{TKey: int} : array{TKey: string})
     */
    public function conditionalValue($key, bool $flag) {}
}
