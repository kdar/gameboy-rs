package main

import (
	"encoding/hex"
	"fmt"
	"log"
	"os"
	"os/exec"
	"reflect"
	"regexp"
	"time"
)

type Regs struct {
	AF uint16
	BC uint16
	DE uint16
	HL uint16
	SP uint16
	PC uint16
}

func (r *Regs) Set(name string, value uint16) {
	switch name {
	case "AF":
		r.AF = value
	case "BC":
		r.BC = value
	case "DE":
		r.DE = value
	case "HL":
		r.HL = value
	case "SP":
		r.SP = value
	case "PC":
		r.PC = value
	}
}

func main() {
	p1 := exec.Command("target/debug/gameboy-emu", "-b", "res/DMG_ROM.bin", "res/cpu_instrs/cpu_instrs.gb", "--debug")

	stdin1, err := p1.StdinPipe()
	if err != nil {
		log.Fatal(err)
	}
	defer stdin1.Close()

	stdout1, err := p1.StdoutPipe()
	if err != nil {
		log.Fatal(err)
	}
	defer stdout1.Close()

	p1.Stderr = os.Stderr

	p2 := exec.Command("/home/outroot/build/realboy-0.2.2/src/realboy", "-d", "res/cpu_instrs/cpu_instrs.gb")

	stdin2, err := p2.StdinPipe()
	if err != nil {
		log.Fatal(err)
	}
	defer stdin2.Close()

	stdout2, err := p2.StdoutPipe()
	if err != nil {
		log.Fatal(err)
	}
	defer stdout2.Close()

	p2.Stderr = os.Stderr

	time.Sleep(1 * time.Second)
	stdin1.Write([]byte("b 2d2\n"))
	stdin1.Write([]byte("c\n"))
	stdin2.Write([]byte("break 0x2d2\n"))
	stdin2.Write([]byte("step 0xFFFFFFFF\n"))
	time.Sleep(3 * time.Second)

	rx1 := regexp.MustCompile(`(?m:^([AFBCDEHLSPC]{2}):\s+0x(....).*$)`)
	rx2 := regexp.MustCompile(`(?m:^([AFBCDEHLSPC]{2}) = 0x(....).*$)`)
	buf1 := make([]byte, 2*1024)
	buf2 := make([]byte, 2*1024)
	go func() {
		for {
			stdin1.Write([]byte("debug\n"))
			stdin2.Write([]byte("show regs\n"))
			time.Sleep(1 * time.Second)

			nr1, err := stdout1.Read(buf1)
			if err != nil {
				log.Fatal(err)
			}

			reg1 := Regs{}
			for _, v := range rx1.FindAllSubmatch(buf1[:nr1], -1) {
				tmp := make([]byte, 2)
				hex.Decode(tmp, v[2])

				reg1.Set(string(v[1]), uint16(tmp[0])<<8|uint16(tmp[1]))
			}

			nr2, err := stdout2.Read(buf2)
			if err != nil {
				log.Fatal(err)
			}

			reg2 := Regs{}
			for _, v := range rx2.FindAllSubmatch(buf2[:nr2], -1) {
				tmp := make([]byte, 2)
				hex.Decode(tmp, v[2])

				reg2.Set(string(v[1]), uint16(tmp[0])<<8|uint16(tmp[1]))
			}

			if !reflect.DeepEqual(reg1, reg2) {
				fmt.Printf("AF: Got: %04x, Expects: %04x\n", reg1.AF, reg2.AF)
				fmt.Printf("BC: Got: %04x, Expects: %04x\n", reg1.BC, reg2.BC)
				fmt.Printf("DE: Got: %04x, Expects: %04x\n", reg1.DE, reg2.DE)
				fmt.Printf("HL: Got: %04x, Expects: %04x\n", reg1.HL, reg2.HL)
				fmt.Printf("SP: Got: %04x, Expects: %04x\n", reg1.SP, reg2.SP)
				fmt.Printf("PC: Got: %04x, Expects: %04x\n", reg1.PC, reg2.PC)
				os.Exit(0)
			}

			fmt.Printf("%04x\n", reg1.PC)
			stdin1.Write([]byte("s\n"))
			stdin2.Write([]byte("step\n"))
		}
	}()

	if err = p1.Start(); err != nil {
		log.Fatal(err)
	}

	if err = p2.Start(); err != nil {
		log.Fatal(err)
	}

	p1.Wait()
	p2.Wait()
}
