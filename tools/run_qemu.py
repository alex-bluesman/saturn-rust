#!/usr/bin/env python3

# Helper script to start QEMU with Saturn and VMs
# Copyright (c) 2025 Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
#
# Licensed under the MIT License (the "License"); you may not use this file except
# in compliance with the License. You may obtain a copy of the License at
#
# http://opensource.org/licenses/MIT
#
# Unless required by applicable law or agreed to in writing, software distributed 
# under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR 
# CONDITIONS OF ANY KIND, either express or implied. See the License for the 
# specific language governing permissions and limitations under the License.



import argparse
import os
import subprocess

def start_qemu(topfolder, extraparams):
    # QEMU command
    cmdline = (['qemu-system-aarch64'])
    cmdline.extend(['-machine', 'virt,gic_version=3'])
    cmdline.extend(['-machine', 'virtualization=true'])
    cmdline.extend(['-machine', 'type=virt'])
    cmdline.extend(['-cpu', 'cortex-a57'])                  # Use ARMv8 64-bit
    cmdline.extend(['-smp', '4'])                           # Use SMP with 4 cores
    cmdline.extend(['-m', '1024M'])                         # Use 1GB of RAM
    cmdline.extend(['-nographic'])                          # Only console output

    # Saturn kernel
    cmdline.extend(['-device', 'loader,file=' + topfolder + '/saturn,addr=0x7fc00000'])
    cmdline.extend(['-device', 'loader,addr=0x7fc00000,cpu-num=0'])

    cmdline.extend([extraparams])

    print(cmdline)
    subprocess.call('exec ' + ' '.join(cmdline), shell=True)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('-b', '--build', help='set the path to build directory', default=os.getcwd())
    parser.add_argument('-e', '--extra', help='pass extra parameters to QEMU', default='')
    args = parser.parse_args()

    start_qemu(args.build, args.extra)
