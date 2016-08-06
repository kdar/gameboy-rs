package main

/*Hacky tool to compare No$gmb's emulator to mine.
It does this by reading No$gmb memory and finding
the values of all the registers. It then steps through
both debuggers and compares the registers at each step.
If it fails, it prints the PC and the register that is wrong.
*/

import (
	"bytes"
	"encoding/hex"
	"log"
	"os"
	"os/exec"
	"regexp"
	"syscall"
	"time"
	"unsafe"

	"github.com/AllenDang/w32"
	"github.com/micmonay/keybd_event"
	"github.com/mozilla/masche/memaccess"
	"github.com/mozilla/masche/process"
)

var (
	moduser32               = syscall.NewLazyDLL("user32.dll")
	procSetForegroundWindow = moduser32.NewProc("SetForegroundWindow")
	procFindWindowW         = moduser32.NewProc("FindWindowW")
)

func SetForegroundWindow(hwnd w32.HWND) bool {
	ret, _, _ := procSetForegroundWindow.Call(
		uintptr(hwnd))

	return ret != 0
}

func FindWindow(name string) w32.HWND {
	ret, _, _ := procFindWindowW.Call(
		0,
		uintptr(unsafe.Pointer(syscall.StringToUTF16Ptr(name))),
	)
	return w32.HWND(ret)
}

func logErrors(harderror error, softerrors []error) {
	if harderror != nil {
		log.Fatal(harderror)
	}
	for _, soft := range softerrors {
		log.Print(soft)
	}
}

func getBaseAddress(handle uintptr) uintptr {
	procEnumProcessModules := syscall.NewLazyDLL("Psapi.dll").NewProc("EnumProcessModules")
	var cbNeeded uint32
	var modules = make([]unsafe.Pointer, 10)
	if success, _, _ := procEnumProcessModules.Call(uintptr(handle), uintptr(unsafe.Pointer(&modules[0])), 10, uintptr(unsafe.Pointer(&cbNeeded))); success > 0 {
		return uintptr(modules[0])
	}
	return 0
}

func main() {
	proc, harderror, softerrors := process.OpenFromPid(uint(36492))
	logErrors(harderror, softerrors)

	hwnd := FindWindow("No$gmb Debugger")
	kb, err := keybd_event.NewKeyBonding()
	if err != nil {
		panic(err)
	}
	kb.SetKeys(keybd_event.VK_F7)

	p := exec.Command("target\\debug\\gameboy-emu.exe", "-b", "res\\DMG_ROM.bin", "res\\cpu_instrs\\individual\\11-op a,(hl).gb", "--debug")

	stdin, err := p.StdinPipe()
	if err != nil {
		log.Fatal(err)
	}
	defer stdin.Close()

	stdout, err := p.StdoutPipe()
	if err != nil {
		log.Fatal(err)
	}
	defer stdout.Close()

	p.Stderr = os.Stderr

	go func() {
		time.Sleep(2 * time.Second)
		stdin.Write([]byte("b c179\n"))
		stdin.Write([]byte("c\n"))
		time.Sleep(3 * time.Second)

		for {
			stdin.Write([]byte("debug\n"))

			time.Sleep(time.Second * 1)

			SetForegroundWindow(hwnd)
			err = kb.Launching()
			if err != nil {
				panic(err)
			}
			stdin.Write([]byte("s\n"))

			time.Sleep(time.Second)
		}
	}()

	rx := regexp.MustCompile(`(?m:^([AFBCDEHLSPC]{2}):\s+0x(....).*$)`)
	go func() {
		for {
			buf := make([]byte, 32*1024)
			nr, err := stdout.Read(buf)
			if err != nil {
				log.Fatal(err)
			}

			membuf := make([]byte, 14)
			harderror, softerrors = memaccess.CopyMemory(proc, uintptr(getBaseAddress(proc.Handle())+0x601E4), membuf[:12])
			logErrors(harderror, softerrors)

			// fix endianess
			for i := 0; i < len(membuf); i += 2 {
				membuf[i], membuf[i+1] = membuf[i+1], membuf[i]
			}

			for i, v := range rx.FindAllSubmatch(buf[:nr], -1) {
				tmp := make([]byte, 2)
				hex.Decode(tmp, v[2])

				// fmt.Printf("%04x: %s: got: %04x, expected: %04x\n", pc, v[1], tmp, membuf[i*2:(i*2)+2])
				if !bytes.Equal(membuf[i*2:(i*2)+2], tmp) {
					log.Fatalf("%04x: missmatch!: %s: got: %04x, expected: %04x\n", membuf[10:12], v[1], tmp, membuf[i*2:(i*2)+2])
				}
			}
		}
	}()

	if err = p.Start(); err != nil {
		log.Fatal(err)
	}

	p.Wait()
}
