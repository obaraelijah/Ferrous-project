Vagrant.configure("2") do |config|
    config.nfs.functional = false
    config.vm.synced_folder "./", "/vagrant", type: "virtiofs"
  
    config.vm.define "Ferrous", primary: true do |Ferrous|
      Ferrous.vm.hostname = "Ferrous"
      Ferrous.vm.box = "generic/debian11"
      Ferrous.vm.network "forwarded_port", guest: 80, host: 8081
      Ferrous.vm.network :private_network, :ip => '10.13.37.10'
      Ferrous.vm.provider "libvirt" do |vb|
          vb.default_prefix = "sam_Ferrous"
          vb.memory = "2048"
          vb.cpus = "8"
          vb.memorybacking :access, :mode => "shared"
      end
      Ferrous.vm.provision :ansible do |a|
        a.playbook = "vagrant/Ferrous.yml"
      end
  end
  
    config.vm.define "leech" do |leech|
      leech.vm.hostname = "leech"
      leech.vm.box = "generic/debian11"
      leech.vm.network :private_network, :ip => '10.13.37.11'
      leech.vm.provider "libvirt" do |vb|
          vb.default_prefix = "sam_Ferrous"
          vb.memory = "2048"
          vb.cpus = "8"
          vb.memorybacking :access, :mode => "shared"
      end
      leech.vm.provision :ansible do |a|
        a.playbook = "vagrant/leech.yml"
      end
    end
  
    config.vm.define "target" do |target|
      target.vm.hostname = "target"
      target.vm.box = "generic/debian11"
      target.vm.network :private_network, :ip => '10.13.37.99'
      target.vm.provider "libvirt" do |vb|
          vb.default_prefix = "sam_Ferrous"
          vb.memory = "512"
          vb.cpus = "2"
          vb.memorybacking :access, :mode => "shared"
      end
      target.vm.provision :ansible do |a|
        a.playbook = "vagrant/target.yml"
      end
    end
  end