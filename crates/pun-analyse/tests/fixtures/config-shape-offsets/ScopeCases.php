<?php

namespace ConfigShapeOffsets;

use Illuminate\Support\Facades\Config;
use ConfigShapeOffsets\Keys as KeyAlias;

class Keys
{
    public const ZERO = 0;
    public const ONE = 1;
}

/** @template TKey of 0 */
function sourceNames(): void
{
    Config::collection('scopes.aliases.0');
    Config::collection('scopes.aliases.1');
    Config::collection('scopes.aliases.2');
    Config::collection('scopes.functionKey.0');
    Config::collection('scopes.functionKey.1');
    Config::collection('scopes.unsealed.known');
    Config::collection('scopes.unsealed.extra');
    Config::collection('scopes.untyped.anything');
    Config::collection('scopes.valueOnly.anything');
}

class ParentKeys
{
    public const KEY = 2;
}

/** @template TClass of 3 */
class ScopedKeys extends ParentKeys
{
    public const KEY = 0;

    /** @template TMethod of 1 */
    public function sourceNames(): void
    {
        Config::collection('scopes.classKeys.0');
        Config::collection('scopes.classKeys.1');
        Config::collection('scopes.lateKeys.0');
        Config::collection('scopes.parentKeys.2');
        Config::collection('scopes.parentKeys.0');
        Config::collection('scopes.methodKey.1');
        Config::collection('scopes.methodKey.0');
        Config::collection('scopes.classKey.3');
        Config::collection('scopes.classKey.0');
    }
}

trait ReadsConfig
{
    public function fromTrait(): void
    {
        Config::collection('scopes.traitKeys.1');
        Config::collection('scopes.traitKeys.0');
    }
}

class UsesConfig
{
    use ReadsConfig;

    public const KEY = 1;
}
