(function() {var implementors = {};
implementors["openstack"] = [{text:"impl&lt;T:&nbsp;<a class=\"trait\" href=\"openstack/common/trait.ResourceId.html\" title=\"trait openstack::common::ResourceId\">ResourceId</a> + <a class=\"trait\" href=\"openstack/common/trait.Refresh.html\" title=\"trait openstack::common::Refresh\">Refresh</a>&gt; <a class=\"trait\" href=\"waiter/trait.Waiter.html\" title=\"trait waiter::Waiter\">Waiter</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"openstack/struct.Error.html\" title=\"struct openstack::Error\">Error</a>&gt; for <a class=\"struct\" href=\"openstack/common/struct.DeletionWaiter.html\" title=\"struct openstack::common::DeletionWaiter\">DeletionWaiter</a>&lt;T&gt;",synthetic:false,types:["openstack::common::waiter::DeletionWaiter"]},{text:"impl&lt;'server&gt; <a class=\"trait\" href=\"waiter/trait.Waiter.html\" title=\"trait waiter::Waiter\">Waiter</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"openstack/struct.Error.html\" title=\"struct openstack::Error\">Error</a>&gt; for <a class=\"struct\" href=\"openstack/compute/struct.ServerStatusWaiter.html\" title=\"struct openstack::compute::ServerStatusWaiter\">ServerStatusWaiter</a>&lt;'server&gt;",synthetic:false,types:["openstack::compute::servers::ServerStatusWaiter"]},{text:"impl <a class=\"trait\" href=\"waiter/trait.Waiter.html\" title=\"trait waiter::Waiter\">Waiter</a>&lt;<a class=\"struct\" href=\"openstack/compute/struct.Server.html\" title=\"struct openstack::compute::Server\">Server</a>, <a class=\"struct\" href=\"openstack/struct.Error.html\" title=\"struct openstack::Error\">Error</a>&gt; for <a class=\"struct\" href=\"openstack/compute/struct.ServerCreationWaiter.html\" title=\"struct openstack::compute::ServerCreationWaiter\">ServerCreationWaiter</a>",synthetic:false,types:["openstack::compute::servers::ServerCreationWaiter"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
