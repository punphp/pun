<?php
function a1() { try { } catch (A $e) { $o = new class { public $e; }; } }
function a2() { try { } catch (A $e) { X::$e = 1; } }
function a3() { try { } catch (A $e) { X::$$f = 1; } }
function a4() { try { } catch (A $e) { X::${'f'} = 1; } }
function a5() { try { } catch (A $e) { X::$$e = 1; } }
function a6() { try { } catch (A $e) { include 'x.php'; } }
function a7() { try { } catch (A $e) { compact('e'); } }
function a8() { try { } catch (A $e) { \COMPACT(['x', ['e']]); } }
function a9() { try { } catch (A $e) { compact(array('a' => 'e')); } }
function b1() { try { } catch (A $e) { compact(var_name: 'e'); } }
function b2() { try { } catch (A $e) { compact(...['e']); } }
function b3() { try { } catch (A $e) { compact('E', $x, 'f'); } }
function b4() { try { } catch (A $e) { Foo\compact('e'); } }
function b5() { try { } catch (A $e) { $f('e'); } }
function b6() { try { } catch (A $e) { echo $E; } }
function b7() { try { try { } catch (A $e) { } } catch (B $e) { } }
function b8() { try { try { } catch (A $e) { } } catch (B $f) { echo $e; } }
function b9() { try { try { } catch (A $e) { } } finally { echo $e; } }
function c1() { try { } catch (A $e) { } finally { echo $e; } }
function c2() { try { } catch (A) { } catch (B$e) {} }
function c3() { try { } catch (A $e) { echo "$e"; } }
function c5() { try { } catch (A $e) { compact(('e')); } }
function c6() { try { } catch (A $e) { $this->$e; } }
function c7() { try { } catch (A $e) { $x = ${'y'}; } }
switch (1) { case 1: try { } catch (A $e) { } }
function c8() { try { } catch (A $e) { compact([...['e']]); } }
-----
<?php
function a1() { try { } catch (A) { $o = new class { public $e; }; } }
function a2() { try { } catch (A) { X::$e = 1; } }
function a3() { try { } catch (A) { X::$$f = 1; } }
function a4() { try { } catch (A) { X::${'f'} = 1; } }
function a5() { try { } catch (A $e) { X::$$e = 1; } }
function a6() { try { } catch (A $e) { include 'x.php'; } }
function a7() { try { } catch (A $e) { compact('e'); } }
function a8() { try { } catch (A $e) { \COMPACT(['x', ['e']]); } }
function a9() { try { } catch (A $e) { compact(array('a' => 'e')); } }
function b1() { try { } catch (A $e) { compact(var_name: 'e'); } }
function b2() { try { } catch (A $e) { compact(...['e']); } }
function b3() { try { } catch (A) { compact('E', $x, 'f'); } }
function b4() { try { } catch (A) { Foo\compact('e'); } }
function b5() { try { } catch (A) { $f('e'); } }
function b6() { try { } catch (A $e) { echo $E; } }
function b7() { try { try { } catch (A) { } } catch (B) { } }
function b8() { try { try { } catch (A $e) { } } catch (B) { echo $e; } }
function b9() { try { try { } catch (A $e) { } } finally { echo $e; } }
function c1() { try { } catch (A $e) { } finally { echo $e; } }
function c2() { try { } catch (A) { } catch (B) {} }
function c3() { try { } catch (A $e) { echo "$e"; } }
function c5() { try { } catch (A $e) { compact(('e')); } }
function c6() { try { } catch (A $e) { $this->$e; } }
function c7() { try { } catch (A $e) { $x = ${'y'}; } }
switch (1) { case 1: try { } catch (A) { } }
function c8() { try { } catch (A $e) { compact([...['e']]); } }
