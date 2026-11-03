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

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class UpdateSchemaLocation implements Migration
{
    private const string NAMESPACE_URI              = 'http://www.w3.org/2001/XMLSchema-instance';
    private const string LOCAL_NAME_SCHEMA_LOCATION = 'noNamespaceSchemaLocation';

    public function migrate(DOMDocument $document): void
    {}
}
