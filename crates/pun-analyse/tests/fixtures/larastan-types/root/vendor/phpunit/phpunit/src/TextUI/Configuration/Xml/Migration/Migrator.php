<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\XmlConfiguration;

use function assert;
use function str_contains;
use DOMDocument;
use DOMElement;
use PHPUnit\Runner\Version;
use PHPUnit\Util\Xml\Loader as XmlLoader;
use PHPUnit\Util\Xml\XmlException;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Migrator
{
    /**
     * @throws Exception
     * @throws MigrationException
     * @throws XmlException
     */
    public function migrate(string $filename): string
    {}

    private function schemaLocationNeedsUpdate(DOMDocument $document): bool
    {}
}
