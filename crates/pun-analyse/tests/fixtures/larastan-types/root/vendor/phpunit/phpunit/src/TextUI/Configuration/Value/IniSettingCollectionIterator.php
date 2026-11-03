<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Configuration;

use function assert;
use Iterator;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @template-implements Iterator<non-negative-int, IniSetting>
 */
final class IniSettingCollectionIterator implements Iterator
{
    /**
     * @var list<IniSetting>
     */
    private readonly array $iniSettings;

    /**
     * @var non-negative-int
     */
    private int $position = 0;

    public function __construct(IniSettingCollection $iniSettings)
    {}

    public function rewind(): void
    {}

    public function valid(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function key(): int
    {}

    public function current(): IniSetting
    {}

    public function next(): void
    {}
}
