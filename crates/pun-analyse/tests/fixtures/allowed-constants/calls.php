<?php declare(strict_types=1);

namespace AllowedConstants;

use const JSON_BIGINT_AS_STRING as ImportedWrong;
use FilesystemIterator as Files;

const ALIAS = JSON_BIGINT_AS_STRING;
const OTHER_ALIAS = SORT_REGULAR;

class LocalFlags
{
    public const ALIAS = JSON_BIGINT_AS_STRING;
    public static function check(): void
    {
        json_encode([], self::ALIAS);
        array_unique([], self::ALIAS | self::ALIAS);
    }
}

class ChildFlags extends LocalFlags
{
    public static function check(): void
    {
        json_encode([], parent::ALIAS);
        json_encode([], static::ALIAS);
    }
}

class InheritedFiles extends Files {}

function calls(\SplPriorityQueue $queue, Files $files, string $dynamicClass, string $dynamicConstant, int $variable): void
{
    // Same value, different identities. Literal numbers, variables and user aliases are not checked by value.
    json_encode([], JSON_BIGINT_AS_STRING);
    json_encode([], JSON_HEX_AMP);
    json_encode([], SORT_REGULAR);
    json_encode([], 2);
    json_encode([], $variable);
    json_encode([], ALIAS);
    json_encode([], LocalFlags::ALIAS);
    json_encode([], ImportedWrong);
    json_encode([], \JSON_BIGINT_AS_STRING);
    json_encode([], (JSON_BIGINT_AS_STRING));

    json_encode([], JSON_HEX_TAG | JSON_PRETTY_PRINT);
    json_encode([], JSON_HEX_TAG | JSON_BIGINT_AS_STRING | SORT_REGULAR);
    json_encode([], JSON_INVALID_UTF8_SUBSTITUTE | JSON_INVALID_UTF8_IGNORE);
    json_encode([], JSON_PARTIAL_OUTPUT_ON_ERROR | JSON_THROW_ON_ERROR);
    json_encode([], JSON_THROW_ON_ERROR | JSON_THROW_ON_ERROR);
    json_encode([], JSON_INVALID_UTF8_IGNORE | ALIAS);
    json_encode([], JSON_BIGINT_AS_STRING | 0);
    json_encode([], JSON_BIGINT_AS_STRING & JSON_HEX_TAG);
    json_encode([], JSON_BIGINT_AS_STRING ^ JSON_HEX_TAG);

    array_unique([], SORT_REGULAR | SORT_NUMERIC);
    array_unique([], ALIAS | OTHER_ALIAS);
    array_unique([], JSON_HEX_TAG | JSON_BIGINT_AS_STRING);
    array_unique([], SORT_NUMERIC);
    array_unique([], DATE_ATOM);

    // The fallback applies only to other built-in parameters covered by the policy types, not mixed or bool.
    json_encode(JSON_BIGINT_AS_STRING, JSON_HEX_TAG, JSON_PRETTY_PRINT);
    json_decode('[]', false, JSON_BIGINT_AS_STRING);
    json_decode('[]', JSON_OBJECT_AS_ARRAY);
    json_decode('[]', false, ALIAS);
    strlen(JSON_HEX_TAG);

    // Named arguments are bound before checking, but descriptions keep the source argument's position/name.
    json_encode(flags: JSON_BIGINT_AS_STRING, value: [], depth: JSON_HEX_TAG);
    json_decode(depth: JSON_BIGINT_AS_STRING, json: '[]', flags: JSON_HEX_TAG);
    json_encode(value: [], unknown: JSON_BIGINT_AS_STRING);
    json_encode(...[[], JSON_BIGINT_AS_STRING]);
    json_encode(...['flags' => JSON_BIGINT_AS_STRING, 'value' => []]);
    $arguments = [[], JSON_BIGINT_AS_STRING];
    json_encode(...$arguments);

    \DateTimeZone::listIdentifiers(JSON_HEX_TAG);
    \DateTimeZone::listIdentifiers(timezoneGroup: JSON_HEX_TAG);
    $queue->setExtractFlags(\PDO::FETCH_ASSOC);
    $queue->setExtractFlags(\SplPriorityQueue::EXTR_DATA | \SplPriorityQueue::EXTR_PRIORITY);
    $files->setFlags(JSON_HEX_TAG);
    $files->setFlags(Files::CURRENT_AS_FILEINFO | Files::CURRENT_AS_PATHNAME);
    $files->setFlags(InheritedFiles::SKIP_DOTS);
    new Files('.', JSON_HEX_TAG);
    new \RecursiveDirectoryIterator('.', \RecursiveDirectoryIterator::SKIP_DOTS);
    new \Attribute(JSON_BIGINT_AS_STRING);

    json_encode([], $dynamicClass::FETCH_ASSOC);
    json_encode([], \PDO::{$dynamicConstant});
    json_encode([], \PDO::class);
    json_encode([], MissingClass::FLAG);
    json_encode([], Files::MISSING);
    json_encode([], MISSING_CONSTANT | JSON_BIGINT_AS_STRING);

    $encode = 'json_encode';
    $encode([], JSON_BIGINT_AS_STRING);
    $encode([], JSON_HEX_TAG, JSON_PRETTY_PRINT);
    echo call_user_func('json_encode', [], JSON_BIGINT_AS_STRING);
    echo call_user_func_array('json_encode', [[], JSON_BIGINT_AS_STRING]);
    echo call_user_func('json_encode', [], JSON_HEX_TAG, JSON_PRETTY_PRINT);
    echo call_user_func_array('json_encode', [[], JSON_HEX_TAG, JSON_PRETTY_PRINT]);
    $set = [$files, 'setFlags'];
    $set(JSON_HEX_TAG);
    $list = ['DateTimeZone', 'listIdentifiers'];
    $list(JSON_HEX_TAG);
}

// A local declaration shadows the global built-in; use const imports retain the imported identity instead.
namespace AllowedConstants\Shadow;
const JSON_BIGINT_AS_STRING = 2;
\json_encode([], JSON_BIGINT_AS_STRING);
\json_encode([], \JSON_BIGINT_AS_STRING);
\json_encode([], true);
\json_encode([], null);
\json_encode([], false);
\json_encode([], \true);

function userFlags(mixed $value, int $flags): string { return ''; }

function policyUnions(bool $choose): void
{
    userFlags([], \JSON_BIGINT_AS_STRING);
    $withUser = $choose ? 'json_encode' : __NAMESPACE__.'\\userFlags';
    echo $withUser([], \JSON_BIGINT_AS_STRING);
    $withClosure = $choose ? 'json_encode' : static fn (mixed $value, int $flags): string => '';
    echo $withClosure([], \JSON_BIGINT_AS_STRING);
}

class NotAGlobalConstant
{
    public static function define(): void { define('ALLOWED_CONSTANT_INSIDE_CLASS', 2); }
}
\json_encode([], \ALLOWED_CONSTANT_INSIDE_CLASS);

/** @param resource $handle */
function referenceFallback($handle): void
{
    flock($handle, LOCK_SH, JSON_HEX_TAG);
}

// Inherited user aliases still count as two references for a single-constant parameter.
\array_unique([], \AllowedConstants\ChildFlags::ALIAS | \AllowedConstants\ChildFlags::ALIAS);

namespace zzzAllowedConstants;

function userFlags(mixed $value, int $flags): string { return ''; }
function voidCallback(): void {}

function combinedAndReferencePolicies(bool $choose, \SysvMessageQueue $queue): void
{
    // With this function name the built-in is combined before the user acceptor, not after it.
    $withUser = $choose ? 'json_encode' : __NAMESPACE__.'\\userFlags';
    echo $withUser([], \JSON_BIGINT_AS_STRING);
    // Creates-variable parameters skip ordinary type acceptance, but not the constant fallback.
    \msg_receive($queue, 0, \JSON_HEX_TAG, 1024, $message, false, \MSG_IPC_NOWAIT);
    $result = voidCallback();
}

// Native enum cases are built-in class-constant identities too; project enum cases remain user constants.
enum UserMode { case HalfEven; }
\json_encode([], \RoundingMode::HalfEven);
\round(1.2, 0, \RoundingMode::HalfEven);
\json_encode([], UserMode::HalfEven);
