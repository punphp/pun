<?php

/**
 * This file is part of the ramsey/uuid library
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 *
 * @copyright Copyright (c) Ben Ramsey <ben@benramsey.com>
 * @license http://opensource.org/licenses/MIT MIT
 */

declare(strict_types=1);

namespace Ramsey\Uuid\Provider\Dce;

use Ramsey\Uuid\Exception\DceSecurityException;
use Ramsey\Uuid\Provider\DceSecurityProviderInterface;
use Ramsey\Uuid\Type\Integer as IntegerObject;

use function escapeshellarg;
use function preg_split;
use function str_getcsv;
use function strrpos;
use function strtolower;
use function strtoupper;
use function substr;
use function trim;

use const PREG_SPLIT_NO_EMPTY;

/**
 * SystemDceSecurityProvider retrieves the user or group identifiers from the system
 */
class SystemDceSecurityProvider implements DceSecurityProviderInterface
{
    /**
     * @throws DceSecurityException if unable to get a user identifier
     *
     * @inheritDoc
     */
    public function getUid(): IntegerObject
    {}

    /**
     * @throws DceSecurityException if unable to get a group identifier
     *
     * @inheritDoc
     */
    public function getGid(): IntegerObject
    {}

    /**
     * Returns the UID from the system
     */
    private function getSystemUid(): string
    {}

    /**
     * Returns the GID from the system
     */
    private function getSystemGid(): string
    {}

    /**
     * Returns true if shell_exec() is available for use
     */
    private function hasShellExec(): bool
    {}

    /**
     * Returns the PHP_OS string
     */
    private function getOs(): string
    {}

    /**
     * Returns the user identifier for a user on a Windows system
     *
     * Windows does not have the same concept as an effective POSIX UID for the running script. Instead, each user is
     * uniquely identified by an SID (security identifier). The SID includes three 32-bit unsigned integers that make up
     * a unique domain identifier, followed by an RID (relative identifier) that we will use as the UID. The primary
     * caveat is that this UID may not be unique to the system, since it is, instead, unique to the domain.
     *
     * @link https://www.lifewire.com/what-is-an-sid-number-2626005 What Is an SID Number?
     * @link https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/81d92bba-d22b-4a8c-908a-554ab29148ab Well-known SID Structures
     * @link https://learn.microsoft.com/en-us/windows-server/identity/ad-ds/manage/understand-security-identifiers#well-known-sids Well-known SIDs
     * @link https://www.windows-commandline.com/get-sid-of-user/ Get SID of user
     */
    private function getWindowsUid(): string
    {}

    /**
     * Returns a group identifier for a user on a Windows system
     *
     * Since Windows does not have the same concept as an effective POSIX GID for the running script, we will get the
     * local group memberships for the user running the script. Then, we will get the SID (security identifier) for the
     * first group that appears in that list. Finally, we will return the RID (relative identifier) for the group and
     * use that as the GID.
     *
     * @link https://www.windows-commandline.com/list-of-user-groups-command-line/ List of user groups command line
     */
    private function getWindowsGid(): string
    {}
}
