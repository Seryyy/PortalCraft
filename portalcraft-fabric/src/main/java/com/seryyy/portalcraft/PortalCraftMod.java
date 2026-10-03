package com.seryyy.portalcraft;

import net.fabricmc.api.ModInitializer;
import net.fabricmc.fabric.api.networking.v1.ServerPlayConnectionEvents;
import net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking;
import net.minecraft.server.network.ServerPlayerEntity;
import net.minecraft.text.Text;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class PortalCraftMod implements ModInitializer {
    public static final String MOD_ID = "portalcraft";
    public static final Logger LOGGER = LoggerFactory.getLogger(MOD_ID);
    
    // Network channel for PortalCraft packets
    public static final net.minecraft.util.Identifier PORTALCRAFT_CHANNEL = 
        new net.minecraft.util.Identifier(MOD_ID, "main");

    @Override
    public void onInitialize() {
        LOGGER.info("Initializing PortalCraft Fabric Mod");
        
        // Register network channel
        ServerPlayNetworking.registerGlobalReceiver(PORTALCRAFT_CHANNEL, (server, player, handler, buf, responseSender) -> {
            // Handle incoming packets from bridge
            handlePacket(player, buf);
        });
        
        // Track player connections
        ServerPlayConnectionEvents.JOIN.register((handler, sender, server) -> {
            ServerPlayerEntity player = handler.getPlayer();
            LOGGER.info("Player joined: {}", player.getName().getString());
            // Request initial sync
            sendHello(player);
        });
        
        ServerPlayConnectionEvents.DISCONNECT.register((handler, server) -> {
            ServerPlayerEntity player = handler.getPlayer();
            LOGGER.info("Player disconnected: {}", player.getName().getString());
        });
        
        LOGGER.info("PortalCraft initialized successfully");
    }
    
    private void handlePacket(ServerPlayerEntity player, net.minecraft.network.PacketByteBuf buf) {
        // TODO: Decode bincode message from bridge
        // For now, just log
        LOGGER.debug("Received packet from bridge for player: {}", player.getName().getString());
    }
    
    private void sendHello(ServerPlayerEntity player) {
        net.minecraft.network.PacketByteBuf buf = net.minecraft.network.PacketByteBuf.create();
        // TODO: Encode Hello message
        ServerPlayNetworking.send(player, PORTALCRAFT_CHANNEL, buf);
    }
    
    // Public API for other classes
    public static void sendToBridge(ServerPlayerEntity player, net.minecraft.network.PacketByteBuf buf) {
        ServerPlayNetworking.send(player, PORTALCRAFT_CHANNEL, buf);
    }
}