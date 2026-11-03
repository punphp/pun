<?php

namespace Illuminate\Console;

use Illuminate\Console\Contracts\NewLineAware;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Console\Question\Question;
use Symfony\Component\Console\Style\SymfonyStyle;

class OutputStyle extends SymfonyStyle implements NewLineAware
{
    /**
     * The output instance.
     *
     * @var \Symfony\Component\Console\Output\OutputInterface
     */
    private $output;

    /**
     * The number of trailing new lines written by the last output.
     *
     * This is initialized as 1 to account for the new line written by the shell after executing a command.
     *
     * @var int
     */
    protected $newLinesWritten = 1;

    /**
     * If the last output written wrote a new line.
     *
     * @var bool
     *
     * @deprecated use $newLinesWritten
     */
    protected $newLineWritten = false;

    /**
     * Create a new Console OutputStyle instance.
     *
     * @param  \Symfony\Component\Console\Input\InputInterface  $input
     * @param  \Symfony\Component\Console\Output\OutputInterface  $output
     */
    public function __construct(InputInterface $input, OutputInterface $output)
    {}

    /**
     * {@inheritdoc}
     */
    #[\Override]
    public function askQuestion(Question $question): mixed
    {}

    /**
     * {@inheritdoc}
     */
    #[\Override]
    public function write(string|iterable $messages, bool $newline = false, int $options = 0): void
    {}

    /**
     * {@inheritdoc}
     */
    #[\Override]
    public function writeln(string|iterable $messages, int $type = self::OUTPUT_NORMAL): void
    {}

    /**
     * {@inheritdoc}
     */
    #[\Override]
    public function newLine(int $count = 1): void
    {}

    /**
     * {@inheritdoc}
     */
    public function newLinesWritten()
    {}

    /**
     * {@inheritdoc}
     *
     * @deprecated use newLinesWritten
     */
    public function newLineWritten()
    {}

    /**
     * Count the number of trailing new lines in a string.
     *
     * @param  string|iterable<string>  $messages
     * @return int
     */
    protected function trailingNewLineCount($messages)
    {}

    /**
     * Returns whether verbosity is quiet (-q).
     *
     * @return bool
     */
    public function isQuiet(): bool
    {}

    /**
     * Returns whether verbosity is verbose (-v).
     *
     * @return bool
     */
    public function isVerbose(): bool
    {}

    /**
     * Returns whether verbosity is very verbose (-vv).
     *
     * @return bool
     */
    public function isVeryVerbose(): bool
    {}

    /**
     * Returns whether verbosity is debug (-vvv).
     *
     * @return bool
     */
    public function isDebug(): bool
    {}

    /**
     * Get the underlying Symfony output implementation.
     *
     * @return \Symfony\Component\Console\Output\OutputInterface
     */
    public function getOutput()
    {}
}
