<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report\Html;

use const PHP_OS_FAMILY;
use function assert;
use function fclose;
use function fflush;
use function fread;
use function fwrite;
use function implode;
use function preg_replace;
use function proc_close;
use function proc_open;
use function sprintf;
use function str_contains;
use function stream_select;
use function strlen;
use function substr;
use SebastianBergmann\CodeCoverage\Data\ProcessedFunctionCoverageData;
use SebastianBergmann\CodeCoverage\Data\ProcessedPathCoverageData;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class ControlFlowGraph
{
    /**
     * Branch identifier used by Xdebug for the exit of a function (XDEBUG_BRANCH_EXIT in xdebug_branch_info.h).
     */
    public const int XDEBUG_EXIT_BRANCH = 2147483645;

    private const int DOT_TIMEOUT_SECONDS = 30;
    private ?bool $dotAvailable           = null;

    /** @var null|resource */
    private $process;

    /** @var array<int, resource> */
    private array $pipes = [];
    private readonly string $dotBinary;

    public function __construct(string $dotBinary = 'dot')
    {}

    public function __destruct()
    {}

    /**
     * @param null|array<int, ProcessedPathCoverageData> $paths
     */
    public function renderSvg(string $methodName, ProcessedFunctionCoverageData $methodData, ?array $paths = null): string
    {}

    /**
     * Generates the graph in Graphviz DOT format.
     *
     * Nodes and edges carry "covered" / "uncovered" classes instead of color
     * literals; the report's stylesheet maps them to the configured color
     * scheme in both light mode and dark mode.
     *
     * The graph carries an identifier derived from the name of the method,
     * which dot also uses as the prefix of the identifiers it generates for
     * the nodes and which is the prefix of the identifiers of the edges.
     * All graphs of a report and all of their elements therefore have
     * distinct identifiers.
     *
     * @param null|array<int, ProcessedPathCoverageData> $paths
     */
    public function generateDot(string $methodName, ProcessedFunctionCoverageData $methodData, ?array $paths = null): string
    {}

    /**
     * @param null|array<int, ProcessedPathCoverageData> $paths
     *
     * @return array<string, list<string>>
     */
    private function buildEdgePathClasses(ProcessedFunctionCoverageData $methodData, ?array $paths): array
    {}

    /**
     * dot spends most of its time initializing itself (plugins, font
     * configuration), not laying out a graph. All graphs of a report are
     * therefore piped through a single dot process, which emits the SVG of
     * each graph as soon as that graph has been read.
     */
    /**
     * The identifier of the graph: the name of the method with every
     * character that is not a letter, a digit, or an underscore replaced by
     * a hyphen, so that it is usable as an HTML identifier as well as in a
     * CSS selector.
     *
     * @return non-empty-string
     */
    private function id(string $methodName): string
    {}

    /**
     * @param non-empty-string $id
     */
    private function dotToSvg(string $dot, string $id): string
    {}

    private function startDot(): bool
    {}

    private function stopDot(): void
    {}
}
