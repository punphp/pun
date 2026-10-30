<?php

namespace WrongConfigScope;

use DateTimeInterface as KeyAlias;

/**
 * @return array{
 *   aliases: array{KeyAlias::ZERO: string, KeyAlias::ONE: array<int>, string},
 *   classKeys: array{self::KEY: string},
 *   lateKeys: array{static::KEY: string},
 *   parentKeys: array{parent::KEY: string},
 *   traitKeys: array{self::KEY: string},
 *   functionKey: array{TKey: string},
 *   methodKey: array{TMethod: string},
 *   classKey: array{TClass: string},
 *   unsealed: array{known: array<int>, ...<string, string>},
 *   untyped: array{...},
 *   valueOnly: array{...<string>}
 * }
 */
return [];
