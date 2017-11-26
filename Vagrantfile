# -*- mode: ruby -*-
# vi: set ft=ruby ts=2 sw=2 expandtab :

PROJECT = "rust_sentence_aligner"

ENV['VAGRANT_NO_PARALLEL'] = 'yes'
ENV['VAGRANT_DEFAULT_PROVIDER'] = 'docker'
Vagrant.configure(2) do |config|

  config.vm.define "db" do |db|
    db.vm.provider "docker" do |d|
      d.image = "postgres:9.5"
      d.name = "#{PROJECT}_db"
      d.env = {
        "POSTGRES_PASSWORD" => "vagrant",
        "POSTGRES_USER" => "vagrant",
        "POSTGRES_DB" => "vagrant",
      }
    end
  end

  config.ssh.insert_key = false
  config.vm.define "dev", primary: true do |app|
    app.vm.provider "docker" do |d|
      d.image = "allansimon/docker-dev-rust"

      d.name = "#{PROJECT}_dev"

      d.link "#{PROJECT}_db:db"

      d.has_ssh = true

      d.env = {
        "HOST_USER_UID" => Process.euid,

        "DB_USER" => "vagrant",
        "DB_PASSWORD" => "vagrant",
        "DB_HOST" => "db",
        "DB_NAME" => "vagrant",
      }
    end
    app.ssh.username = "vagrant"

    # libssl-dev is required by reqwest when testing
    app.vm.provision "installs", "type": "shell" do |installs|
      installs.inline = "
        sudo apt-get update
        sudo apt-get install pkg-config libssl-dev -y
      "
    end
  end
end
