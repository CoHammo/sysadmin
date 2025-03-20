export class Exit {
	constructor(msgpack: Array<any>) {
		this.exit_code = msgpack[0];
		this.real_command = msgpack[1];
		this.out = msgpack[2];
		if (msgpack[3] != null) {
			this.packages_response = new PackagesResponse(msgpack[3]);
		}
		if (msgpack[4] != null) {
			this.error_message = msgpack[4];
		}
	}

	exit_code: number;
	real_command: string;
	out: string;
	packages_response?: PackagesResponse;
	error_message?: string;
}

class PackagesResponse {
	constructor(msgpack: Array<any>) {
		this.subcommand = msgpack[0];
		this.messages = msgpack[1];
		this.packages = [];
		for (let i = 0; i < msgpack[2].length; i++) {
			this.packages.push(new Package(msgpack[2][i]));
		}
		this.packages_size = msgpack[3];
		this.packages_length = msgpack[4];
		this.uninstalled_packages = [];
		for (let i = 0; i < msgpack[5].length; i++) {
			this.uninstalled_packages.push(new Package(msgpack[5][i]));
		}
		this.uninstalled_packages_size = msgpack[6];
		this.uninstalled_packages_length = msgpack[7];
	}
	subcommand: string;
	messages: Array<string>;
	packages: Array<Package>;
	packages_size?: string;
	packages_length?: number;
	uninstalled_packages: Array<Package>;
	uninstalled_packages_size?: string;
	uninstalled_packages_length?: number;
}

class Package {
	constructor(msgpack: Array<any>) {
		this.name = msgpack[0];
		this.arch = msgpack[1];
		this.version = msgpack[2];
		this.old_version = msgpack[3];
		this.repository = msgpack[4];
		this.size = msgpack[5];
		this.download = msgpack[6];
		this.description = msgpack[7];
		this.url = msgpack[8];
		this.license = msgpack[9];
		this.installed = msgpack[10];
	}

	name: string;
	arch: string;
	version?: string;
	old_version?: string;
	repository?: string;
	size?: string;
	download?: string;
	description?: string;
	url?: string;
	license?: string;
	installed?: boolean;
}
