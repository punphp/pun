<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Style;

use Symfony\Component\Console\Exception\InvalidArgumentException;
use Symfony\Component\Console\Exception\RuntimeException;
use Symfony\Component\Console\Formatter\OutputFormatter;
use Symfony\Component\Console\Helper\Helper;
use Symfony\Component\Console\Helper\OutputWrapper;
use Symfony\Component\Console\Helper\ProgressBar;
use Symfony\Component\Console\Helper\SymfonyQuestionHelper;
use Symfony\Component\Console\Helper\Table;
use Symfony\Component\Console\Helper\TableCell;
use Symfony\Component\Console\Helper\TableSeparator;
use Symfony\Component\Console\Helper\TreeHelper;
use Symfony\Component\Console\Helper\TreeNode;
use Symfony\Component\Console\Helper\TreeStyle;
use Symfony\Component\Console\Input\File\InputFile;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Output\ConsoleOutputInterface;
use Symfony\Component\Console\Output\ConsoleSectionOutput;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Console\Output\TrimmedBufferOutput;
use Symfony\Component\Console\Question\ChoiceQuestion;
use Symfony\Component\Console\Question\ConfirmationQuestion;
use Symfony\Component\Console\Question\FileQuestion;
use Symfony\Component\Console\Question\Question;
use Symfony\Component\Console\Terminal;
use Symfony\Contracts\EventDispatcher\EventDispatcherInterface;

/**
 * Output decorator helpers for the Symfony Style Guide.
 *
 * @author Kevin Bond <kevinbond@gmail.com>
 */
class SymfonyStyle extends OutputStyle
{
    public const MAX_LINE_LENGTH = 120;

    private SymfonyQuestionHelper $questionHelper;
    private ProgressBar $progressBar;
    private int $lineLength;
    private TrimmedBufferOutput $bufferedOutput;

    public function __construct(
        private InputInterface $input,
        private OutputInterface $output,
        private ?EventDispatcherInterface $dispatcher = null,
    ) {}

    /**
     * Formats a message as a block of text.
     */
    public function block(string|array $messages, ?string $type = null, ?string $style = null, string $prefix = ' ', bool $padding = false, bool $escape = true): void
    {}

    public function title(string $message): void
    {}

    public function section(string $message): void
    {}

    public function listing(array $elements): void
    {}

    public function text(string|array $message): void
    {}

    /**
     * Formats a command comment.
     */
    public function comment(string|array $message): void
    {}

    public function success(string|array $message): void
    {}

    public function error(string|array $message): void
    {}

    public function warning(string|array $message): void
    {}

    public function note(string|array $message): void
    {}

    /**
     * Formats an info message.
     */
    public function info(string|array $message): void
    {}

    public function caution(string|array $message): void
    {}

    /**
     * Formats a message as an outlined block of text.
     *
     * Unlike block(), this renders colored borders instead of colored backgrounds,
     * improving readability across terminal color schemes.
     */
    public function outlineBlock(string|array $messages, ?string $type = null, ?string $style = null, bool $padding = true, bool $escape = true): void
    {}

    public function outlineSuccess(string|array $message): void
    {}

    public function outlineError(string|array $message): void
    {}

    public function outlineWarning(string|array $message): void
    {}

    public function outlineNote(string|array $message): void
    {}

    public function outlineInfo(string|array $message): void
    {}

    public function outlineCaution(string|array $message): void
    {}

    public function table(array $headers, array $rows): void
    {}

    /**
     * Formats a horizontal table.
     */
    public function horizontalTable(array $headers, array $rows): void
    {}

    /**
     * Formats a list of key/value horizontally.
     *
     * Each row can be one of:
     * * 'A title'
     * * ['key' => 'value']
     * * new TableSeparator()
     */
    public function definitionList(string|array|TableSeparator ...$list): void
    {}

    public function ask(string $question, ?string $default = null, ?callable $validator = null): mixed
    {}

    public function askHidden(string $question, ?callable $validator = null): mixed
    {}

    public function confirm(string $question, bool $default = true): bool
    {}

    public function choice(string $question, array $choices, mixed $default = null, bool $multiSelect = false): mixed
    {}

    public function askFile(string $question): ?InputFile
    {}

    /**
     * @param string|null $format
     */
    public function progressStart(int $max = 0 /* , ?string $format = null */): void
    { func_get_args(); }

    public function progressAdvance(int $step = 1): void
    {}

    public function progressFinish(): void
    {}

    /**
     * @param string|null $format
     */
    public function createProgressBar(int $max = 0 /* , ?string $format = null */): ProgressBar
    { func_get_args(); }

    /**
     * @see ProgressBar::iterate()
     *
     * @template TKey
     * @template TValue
     *
     * @param iterable<TKey, TValue> $iterable
     * @param int|null               $max      Number of steps to complete the bar (0 if indeterminate), if null it will be inferred from $iterable
     * @param string|null            $format   A ProgressBar format string (e.g. ' %current%/%max% [%bar%] %memory:6s%'); null uses the default format
     *
     * @return iterable<TKey, TValue>
     */
    public function progressIterate(iterable $iterable, ?int $max = null /* , ?string $format = null */): iterable
    { yield; func_get_args(); }

    public function askQuestion(Question $question): mixed
    {}

    public function writeln(string|iterable $messages, int $type = self::OUTPUT_NORMAL): void
    {}

    public function write(string|iterable $messages, bool $newline = false, int $type = self::OUTPUT_NORMAL): void
    {}

    public function newLine(int $count = 1): void
    {}

    /**
     * Returns a new instance which makes use of stderr if available.
     */
    public function getErrorStyle(): self
    {}

    public function createTable(): Table
    {}

    private function getProgressBar(): ProgressBar
    {}

    /**
     * @param iterable<string, iterable|string|TreeNode> $nodes
     */
    public function tree(iterable $nodes, string $root = ''): void
    {}

    /**
     * @param iterable<string, iterable|string|TreeNode> $nodes
     */
    public function createTree(iterable $nodes, string $root = ''): TreeHelper
    {}

    private function autoPrependBlock(): void
    {}

    private function autoPrependText(): void
    {}

    private function writeBuffer(string $message, bool $newLine, int $type): void
    {}

    private function createBlock(iterable $messages, ?string $type = null, ?string $style = null, string $prefix = ' ', bool $padding = false, bool $escape = false): array
    {}

    private function createOutlineBlock(array $messages, ?string $type = null, ?string $style = null, bool $padding = false, bool $escape = false): array
    {}
}
