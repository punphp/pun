<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Helper;

use Symfony\Component\Console\Cursor;
use Symfony\Component\Console\Exception\LogicException;
use Symfony\Component\Console\Output\ConsoleOutputInterface;
use Symfony\Component\Console\Output\ConsoleSectionOutput;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Console\Terminal;

/**
 * The ProgressBar provides helpers to display progress output.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Chris Jones <leeked@gmail.com>
 */
final class ProgressBar
{
    public const FORMAT_VERBOSE = 'verbose';
    public const FORMAT_VERY_VERBOSE = 'very_verbose';
    public const FORMAT_DEBUG = 'debug';
    public const FORMAT_NORMAL = 'normal';

    private const FORMAT_VERBOSE_NOMAX = 'verbose_nomax';
    private const FORMAT_VERY_VERBOSE_NOMAX = 'very_verbose_nomax';
    private const FORMAT_DEBUG_NOMAX = 'debug_nomax';
    private const FORMAT_NORMAL_NOMAX = 'normal_nomax';

    private int $barWidth = 28;
    private string $barChar;
    private string $emptyBarChar = '-';
    private string $progressChar = '>';
    private ?string $format = null;
    private ?string $internalFormat = null;
    private ?int $redrawFreq = 1;
    private int $writeCount = 0;
    private float $lastWriteTime = 0;
    private float $minSecondsBetweenRedraws = 0;
    private float $maxSecondsBetweenRedraws = 1;
    private OutputInterface $output;
    private int $step = 0;
    private int $startingStep = 0;
    private ?int $max = null;
    private int $startTime;
    private int $stepWidth;
    private float $percent = 0.0;
    private array $messages = [];
    private bool $overwrite = true;
    private Terminal $terminal;
    private ?string $previousMessage = null;
    private int $lastDrawnStep = 0;
    private ?int $lastDrawnMax = null;
    private bool $maxInferred = false;
    private Cursor $cursor;
    private array $placeholders = [];
    private ?int $oscProgressState = null;
    private ?int $oscProgressPercent = null;

    private static array $formatters;
    private static array $formats;
    private static \WeakMap $activeInstances;
    private static int $pausedCount = 0;

    /**
     * @param int $max Maximum steps (0 if unknown)
     */
    public function __construct(OutputInterface $output, int $max = 0, float $minSecondsBetweenRedraws = 1 / 25)
    {}

    /**
     * Sets a placeholder formatter for a given name, globally for all instances of ProgressBar.
     *
     * This method also allow you to override an existing placeholder.
     *
     * @param string                       $name     The placeholder name (including the delimiter char like %)
     * @param callable(ProgressBar):string $callable A PHP callable
     */
    public static function setPlaceholderFormatterDefinition(string $name, callable $callable): void
    {}

    /**
     * Gets the placeholder formatter for a given name.
     *
     * @param string $name The placeholder name (including the delimiter char like %)
     */
    public static function getPlaceholderFormatterDefinition(string $name): ?callable
    {}

    /**
     * Sets a placeholder formatter for a given name, for this instance only.
     *
     * @param callable(ProgressBar):string $callable A PHP callable
     */
    public function setPlaceholderFormatter(string $name, callable $callable): void
    {}

    /**
     * Gets the placeholder formatter for a given name.
     *
     * @param string $name The placeholder name (including the delimiter char like %)
     */
    public function getPlaceholderFormatter(string $name): ?callable
    {}

    /**
     * Sets a format for a given name.
     *
     * This method also allow you to override an existing format.
     *
     * @param string $name   The format name
     * @param string $format A format string
     */
    public static function setFormatDefinition(string $name, string $format): void
    {}

    /**
     * Gets the format for a given name.
     *
     * @param string $name The format name
     */
    public static function getFormatDefinition(string $name): ?string
    {}

    /**
     * Associates a text with a named placeholder.
     *
     * The text is displayed when the progress bar is rendered but only
     * when the corresponding placeholder is part of the custom format line
     * (by wrapping the name with %).
     *
     * @param string $message The text to associate with the placeholder
     * @param string $name    The name of the placeholder
     */
    public function setMessage(string $message, string $name = 'message'): void
    {}

    public function getMessage(string $name = 'message'): ?string
    {}

    public function getStartTime(): int
    {}

    public function getMaxSteps(): int
    {}

    public function getProgress(): int
    {}

    private function getStepWidth(): int
    {}

    public function getProgressPercent(): float
    {}

    public function getBarOffset(): float
    {}

    public function getEstimated(): float
    {}

    public function getRemaining(): float
    {}

    public function setBarWidth(int $size): void
    {}

    public function getBarWidth(): int
    {}

    public function setBarCharacter(string $char): void
    {}

    public function getBarCharacter(): string
    {}

    public function setEmptyBarCharacter(string $char): void
    {}

    public function getEmptyBarCharacter(): string
    {}

    public function setProgressCharacter(string $char): void
    {}

    public function getProgressCharacter(): string
    {}

    public function setFormat(string $format): void
    {}

    /**
     * Sets the redraw frequency.
     *
     * @param int|null $freq The frequency in steps
     */
    public function setRedrawFrequency(?int $freq): void
    {}

    public function minSecondsBetweenRedraws(float $seconds): void
    {}

    public function maxSecondsBetweenRedraws(float $seconds): void
    {}

    /**
     * Returns an iterator that will automatically update the progress bar when iterated.
     *
     * @template TKey
     * @template TValue
     *
     * @param iterable<TKey, TValue> $iterable
     * @param int|null               $max      Number of steps to complete the bar (0 if indeterminate), if null it will be inferred from $iterable
     *
     * @return iterable<TKey, TValue>
     */
    public function iterate(iterable $iterable, ?int $max = null): iterable
    { yield; }

    /**
     * Starts the progress output.
     *
     * @param int|null $max     Number of steps to complete the bar (0 if indeterminate), null to leave unchanged
     * @param int      $startAt The starting point of the bar (useful e.g. when resuming a previously started bar)
     */
    public function start(?int $max = null, int $startAt = 0): void
    {}

    /**
     * Advances the progress output X steps.
     *
     * @param int $step Number of steps to advance
     */
    public function advance(int $step = 1): void
    {}

    /**
     * Sets whether to overwrite the progressbar, false for new line.
     */
    public function setOverwrite(bool $overwrite): void
    {}

    public function setProgress(int $step): void
    {}

    public function setMaxSteps(?int $max): void
    {}

    /**
     * Finishes the progress output.
     */
    public function finish(): void
    {}

    /**
     * Outputs the current progress string.
     */
    public function display(): void
    {}

    /**
     * Removes the progress bar from the current line.
     *
     * This is useful if you wish to write some output
     * while a progress bar is running.
     * Call display() to show the progress bar again.
     */
    public function clear(): void
    {}

    private function setRealFormat(string $format): void
    {}

    /**
     * Overwrites a previous message to the output.
     */
    private function overwrite(string $message): void
    {}

    /**
     * Pauses the OSC 9;4 taskbar progress indicator on all active progress bars.
     *
     * Calls are reentrant; each call must be paired with a {@see resumeAll()}.
     * Useful when prompting the user for input while a progress bar is running.
     */
    public static function pauseAll(): void
    {}

    /**
     * Counterpart of {@see pauseAll()}.
     */
    public static function resumeAll(): void
    {}

    private function writeOscProgress(): void
    {}

    private function writeOscProgressReset(): void
    {}

    private function writeOscRaw(string $bytes): void
    {}

    private function determineBestFormat(): string
    {}

    private static function initPlaceholderFormatters(): array
    {}

    private static function initFormats(): array
    {}

    private function buildLine(): string
    {}
}
