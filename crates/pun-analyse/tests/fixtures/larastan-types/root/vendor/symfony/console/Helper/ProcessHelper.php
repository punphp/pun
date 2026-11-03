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

use Symfony\Component\Console\Output\ConsoleOutputInterface;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Process\Exception\ProcessFailedException;
use Symfony\Component\Process\Process;

/**
 * The ProcessHelper class provides helpers to run external processes.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 *
 * @final
 */
class ProcessHelper extends Helper
{
    /**
     * Runs an external process.
     *
     * @param array|Process $cmd      An instance of Process or an array of the command and arguments
     * @param callable|null $callback A PHP callback to run whenever there is some
     *                                output available on STDOUT or STDERR
     */
    public function run(OutputInterface $output, array|Process $cmd, ?string $error = null, ?callable $callback = null, int $verbosity = OutputInterface::VERBOSITY_VERY_VERBOSE): Process
    {}

    /**
     * Runs the process.
     *
     * This is identical to run() except that an exception is thrown if the process
     * exits with a non-zero exit code.
     *
     * @param array|Process $cmd      An instance of Process or a command to run
     * @param callable|null $callback A PHP callback to run whenever there is some
     *                                output available on STDOUT or STDERR
     *
     * @throws ProcessFailedException
     *
     * @see run()
     */
    public function mustRun(OutputInterface $output, array|Process $cmd, ?string $error = null, ?callable $callback = null, int $verbosity = OutputInterface::VERBOSITY_VERY_VERBOSE): Process
    {}

    /**
     * Wraps a Process callback to add debugging output.
     */
    public function wrapCallback(OutputInterface $output, Process $process, ?callable $callback = null): callable
    {}

    private function escapeString(string $str): string
    {}

    public function getName(): string
    {}
}
