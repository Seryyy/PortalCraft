package com.seryyy.portalcraft.client;

import com.seryyy.portalcraft.PortalCraftMod;
import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.networking.v1.ClientPlayNetworking;
import net.minecraft.client.MinecraftClient;
import net.minecraft.text.Text;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class PortalCraftClientMod implements ClientModInitializer {
    public static final Logger LOGGER = LoggerFactory.getLogger("PortalCraftClient");

    @Override
    public void onInitializeClient() {
        LOGGER.info("Initializing PortalCraft Client Mod");
        
        // Register client-side packet handler
        ClientPlayNetworking.registerGlobalReceiver(PortalCraftMod.PORTALCRAFT_CHANNEL, (client, handler, buf, responseSender) -> {
            // Handle packets from server (bridge)
            client.execute(() -> handlePacket(buf));
        });
        
        LOGGER.info("PortalCraft Client initialized");
    }
    
    private void handlePacket(net.minecraft.network.PacketByteBuf buf) {
        // TODO: Decode bincode messages from bridge
        // Handle PlaceBlock, BreakBlock, InventorySync, PlayerState, etc.
        LOGGER.debug("Received packet from bridge on client");
    }
}