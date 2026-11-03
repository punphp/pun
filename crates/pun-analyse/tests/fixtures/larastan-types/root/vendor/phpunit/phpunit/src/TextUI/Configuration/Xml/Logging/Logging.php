<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\XmlConfiguration\Logging;

use PHPUnit\TextUI\XmlConfiguration\Exception;
use PHPUnit\TextUI\XmlConfiguration\Logging\TestDox\Html as TestDoxHtml;
use PHPUnit\TextUI\XmlConfiguration\Logging\TestDox\Text as TestDoxText;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 *
 * @immutable
 */
final readonly class Logging
{
    private ?Junit $junit;
    private ?Otr $otr;
    private ?TeamCity $teamCity;
    private ?TestDoxHtml $testDoxHtml;
    private ?TestDoxText $testDoxText;

    public function __construct(?Junit $junit, ?Otr $otr, ?TeamCity $teamCity, ?TestDoxHtml $testDoxHtml, ?TestDoxText $testDoxText)
    {}

    public function hasJunit(): bool
    {}

    /**
     * @throws Exception
     */
    public function junit(): Junit
    {}

    public function hasOtr(): bool
    {}

    /**
     * @throws Exception
     */
    public function otr(): Otr
    {}

    public function hasTeamCity(): bool
    {}

    /**
     * @throws Exception
     */
    public function teamCity(): TeamCity
    {}

    public function hasTestDoxHtml(): bool
    {}

    /**
     * @throws Exception
     */
    public function testDoxHtml(): TestDoxHtml
    {}

    public function hasTestDoxText(): bool
    {}

    /**
     * @throws Exception
     */
    public function testDoxText(): TestDoxText
    {}
}
