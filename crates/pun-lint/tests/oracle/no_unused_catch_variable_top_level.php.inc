<?php
try { } catch (A $e) { }
echo $e;
try { } catch (A $e) { }
if ($x) try { } catch (A $e) { }
{ try { } catch (A $e) { } }
-----
<?php
try { } catch (A $e) { }
echo $e;
try { } catch (A) { }
if ($x) try { } catch (A) { }
{ try { } catch (A) { } }
