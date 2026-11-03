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
use function sprintf;
use DOMDocument;
use DOMElement;
use DOMXPath;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
abstract readonly class LogToReportMigration implements Migration
{
    /**
     * @throws MigrationException
     */
    public function migrate(DOMDocument $document): void
    {}

    /**
     * @param list<non-empty-string> $attributes
     */
    protected function migrateAttributes(DOMElement $src, DOMElement $dest, array $attributes): void
    {}

    abstract protected function forType(): string;

    abstract protected function toReportFormat(DOMElement $logNode): DOMElement;

    private function findLogNode(DOMDocument $document): ?DOMElement
    {}
}
