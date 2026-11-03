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

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for sebastian/git-state
 */
final readonly class State
{
    /**
     * @var non-empty-string
     */
    private string $originUrl;

    /**
     * @var non-empty-string
     */
    private string $branch;

    /**
     * @var non-empty-string
     */
    private string $commit;
    private bool $clean;
    private string $status;

    /**
     * @param non-empty-string $originUrl
     * @param non-empty-string $branch
     * @param non-empty-string $commit
     */
    public function __construct(string $originUrl, string $branch, string $commit, bool $clean, string $status)
    {}

    /**
     * @return non-empty-string
     */
    public function originUrl(): string
    {}

    /**
     * @return non-empty-string
     */
    public function branch(): string
    {}

    /**
     * @return non-empty-string
     */
    public function commit(): string
    {}

    public function isClean(): bool
    {}

    public function status(): string
    {}
}
