<?php declare(strict_types=1);
/*
 * This file is part of sebastian/git-state.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\GitState;

use function assert;
use function explode;
use function preg_split;
use function str_contains;
use function str_replace;
use function str_starts_with;
use function trim;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for sebastian/git-state
 */
final readonly class Builder
{
    private GitCommandRunner $runner;

    public function __construct(?GitCommandRunner $runner = null)
    {}

    public function build(): false|State
    {}

    /**
     * Returns the hash of the commit that is checked out, for which, unlike
     * for build(), no origin remote needs to be configured, and false when the
     * working directory is not part of a Git repository, the repository has
     * no commit yet, or Git cannot be run.
     *
     * @return false|non-empty-string
     */
    public function commit(): false|string
    {}
}
