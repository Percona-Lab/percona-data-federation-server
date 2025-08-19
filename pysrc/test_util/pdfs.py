import subprocess

class Pdfs:
    def __init__(self, *args):
        self.command = ['./target/debug/percona-data-federation-server'] + list(args)
        self.process = None

    def start(self):
        if self.process is None:
            self.process = subprocess.Popen(
                self.command,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True # For text output
            )
        else:
            print("Process already started.")

    def is_running(self):
        return self.process is not None and self.process.poll() is None

    def stop(self):
        if self.process:
            self.process.terminate()
            self.process.wait()
            self.process = None
