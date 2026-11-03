<?php

/*
 * This file is part of Composer.
 *
 * (c) Nils Adermann <naderman@naderman.de>
 *     Jordi Boggiano <j.boggiano@seld.be>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Composer\Autoload;

/**
 * ClassLoader implements a PSR-0, PSR-4 and classmap class loader.
 *
 *     $loader = new \Composer\Autoload\ClassLoader();
 *
 *     // register classes with namespaces
 *     $loader->add('Symfony\Component', __DIR__.'/component');
 *     $loader->add('Symfony',           __DIR__.'/framework');
 *
 *     // activate the autoloader
 *     $loader->register();
 *
 *     // to enable searching the include path (eg. for PEAR packages)
 *     $loader->setUseIncludePath(true);
 *
 * In this example, if you try to use a class in the Symfony\Component
 * namespace or one of its children (Symfony\Component\Console for instance),
 * the autoloader will first look for the class under the component/
 * directory, and it will then fallback to the framework/ directory if not
 * found before giving up.
 *
 * This class is loosely based on the Symfony UniversalClassLoader.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Jordi Boggiano <j.boggiano@seld.be>
 * @see    https://www.php-fig.org/psr/psr-0/
 * @see    https://www.php-fig.org/psr/psr-4/
 */
class ClassLoader
{
    /** @var \Closure(string):void */
    private static $includeFile;

    /** @var string|null */
    private $vendorDir;

    // PSR-4
    /**
     * @var array<string, array<string, int>>
     */
    private $prefixLengthsPsr4 = array();
    /**
     * @var array<string, list<string>>
     */
    private $prefixDirsPsr4 = array();
    /**
     * @var list<string>
     */
    private $fallbackDirsPsr4 = array();

    // PSR-0
    /**
     * List of PSR-0 prefixes
     *
     * Structured as array('F (first letter)' => array('Foo\Bar (full prefix)' => array('path', 'path2')))
     *
     * @var array<string, array<string, list<string>>>
     */
    private $prefixesPsr0 = array();
    /**
     * @var list<string>
     */
    private $fallbackDirsPsr0 = array();

    /** @var bool */
    private $useIncludePath = false;

    /**
     * @var array<string, string>
     */
    private $classMap = array();

    /** @var bool */
    private $classMapAuthoritative = false;

    /**
     * @var array<string, bool>
     */
    private $missingClasses = array();

    /** @var string|null */
    private $apcuPrefix;

    /**
     * @var array<string, self>
     */
    private static $registeredLoaders = array();

    /**
     * @param string|null $vendorDir
     */
    public function __construct($vendorDir = null)
    {}

    /**
     * @return array<string, list<string>>
     */
    public function getPrefixes()
    {}

    /**
     * @return array<string, list<string>>
     */
    public function getPrefixesPsr4()
    {}

    /**
     * @return list<string>
     */
    public function getFallbackDirs()
    {}

    /**
     * @return list<string>
     */
    public function getFallbackDirsPsr4()
    {}

    /**
     * @return array<string, string> Array of classname => path
     */
    public function getClassMap()
    {}

    /**
     * @param array<string, string> $classMap Class to filename map
     *
     * @return void
     */
    public function addClassMap(array $classMap)
    {}

    /**
     * Registers a set of PSR-0 directories for a given prefix, either
     * appending or prepending to the ones previously set for this prefix.
     *
     * @param string              $prefix  The prefix
     * @param list<string>|string $paths   The PSR-0 root directories
     * @param bool                $prepend Whether to prepend the directories
     *
     * @return void
     */
    public function add($prefix, $paths, $prepend = false)
    {}

    /**
     * Registers a set of PSR-4 directories for a given namespace, either
     * appending or prepending to the ones previously set for this namespace.
     *
     * @param string              $prefix  The prefix/namespace, with trailing '\\'
     * @param list<string>|string $paths   The PSR-4 base directories
     * @param bool                $prepend Whether to prepend the directories
     *
     * @throws \InvalidArgumentException
     *
     * @return void
     */
    public function addPsr4($prefix, $paths, $prepend = false)
    {}

    /**
     * Registers a set of PSR-0 directories for a given prefix,
     * replacing any others previously set for this prefix.
     *
     * @param string              $prefix The prefix
     * @param list<string>|string $paths  The PSR-0 base directories
     *
     * @return void
     */
    public function set($prefix, $paths)
    {}

    /**
     * Registers a set of PSR-4 directories for a given namespace,
     * replacing any others previously set for this namespace.
     *
     * @param string              $prefix The prefix/namespace, with trailing '\\'
     * @param list<string>|string $paths  The PSR-4 base directories
     *
     * @throws \InvalidArgumentException
     *
     * @return void
     */
    public function setPsr4($prefix, $paths)
    {}

    /**
     * Turns on searching the include path for class files.
     *
     * @param bool $useIncludePath
     *
     * @return void
     */
    public function setUseIncludePath($useIncludePath)
    {}

    /**
     * Can be used to check if the autoloader uses the include path to check
     * for classes.
     *
     * @return bool
     */
    public function getUseIncludePath()
    {}

    /**
     * Turns off searching the prefix and fallback directories for classes
     * that have not been registered with the class map.
     *
     * @param bool $classMapAuthoritative
     *
     * @return void
     */
    public function setClassMapAuthoritative($classMapAuthoritative)
    {}

    /**
     * Should class lookup fail if not found in the current class map?
     *
     * @return bool
     */
    public function isClassMapAuthoritative()
    {}

    /**
     * APCu prefix to use to cache found/not-found classes, if the extension is enabled.
     *
     * @param string|null $apcuPrefix
     *
     * @return void
     */
    public function setApcuPrefix($apcuPrefix)
    {}

    /**
     * The APCu prefix in use, or null if APCu caching is not enabled.
     *
     * @return string|null
     */
    public function getApcuPrefix()
    {}

    /**
     * Registers this instance as an autoloader.
     *
     * @param bool $prepend Whether to prepend the autoloader or not
     *
     * @return void
     */
    public function register($prepend = false)
    {}

    /**
     * Unregisters this instance as an autoloader.
     *
     * @return void
     */
    public function unregister()
    {}

    /**
     * Loads the given class or interface.
     *
     * @param  string    $class The name of the class
     * @return true|null True if loaded, null otherwise
     */
    public function loadClass($class)
    {}

    /**
     * Finds the path to the file where the class is defined.
     *
     * @param string $class The name of the class
     *
     * @return string|false The path if found, false otherwise
     */
    public function findFile($class)
    {}

    /**
     * Returns the currently registered loaders keyed by their corresponding vendor directories.
     *
     * @return array<string, self>
     */
    public static function getRegisteredLoaders()
    {}

    /**
     * @param  string       $class
     * @param  string       $ext
     * @return string|false
     */
    private function findFileWithExtension($class, $ext)
    {}

    /**
     * @return void
     */
    private static function initializeIncludeClosure()
    {}
}
