<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Command;

use const PHP_EOL;
use function assert;
use function file_put_contents;
use function ksort;
use function sprintf;
use PHPUnit\Framework\TestCase;
use PHPUnit\Runner\Phpt\TestCase as PhptTestCase;
use ReflectionClass;
use XMLWriter;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class ListTestsAsXmlCommand implements Command
{
    /**
     * @var list<PhptTestCase|TestCase>
     */
    private array $tests;
    private string $filename;

    /**
     * @param list<PhptTestCase|TestCase> $tests
     */
    public function __construct(array $tests, string $filename)
    {}

    public function execute(): Result
    {}
}
