MEMORY
{
    /* NXP LPC810 */
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 0x00001000 /* 4 Kbytes */
    RAM (rwx) :  ORIGIN = 0x10000000, LENGTH = 0x00000400 /* 1 Kbyte */
}
